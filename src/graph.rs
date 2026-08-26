use crate::data_consumer::QuoteHistory;
use crate::data_producer::{EXPOSURE_MAX, EXPOSURE_MIN, generation_curve};
use crate::tranche::TrancheOrder;
use plotters::prelude::*;
use std::error::Error;

/// Renders target exposure (x) against expected yield (y), one point per
/// quote, as a scatter plot written to `path` (PNG), with:
/// - `tranches` drawn as translucent vertical bands behind the points,
///   shaded by rate and labeled; quotes that were actually used to price a
///   tranche (its `sample`) are drawn green instead of red;
/// - the noiseless generation curve quotes are sampled around, as a solid
///   black line;
/// - a step line tracing each tranche's rate across its exposure band;
/// - a second line, on a secondary right-hand axis, tracing the running
///   count of quotes whose exposure is at or below each point on the
///   x-axis (an empirical CDF).
pub fn plot_expectations(
    history: &QuoteHistory,
    tranches: &[TrancheOrder],
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let quotes = &history.quotes;
    if quotes.is_empty() {
        return Err("no quotes to plot".into());
    }

    let yields: Vec<f64> = quotes.iter().map(|q| q.expected_yield).collect();
    let exposures: Vec<f64> = quotes.iter().map(|q| q.target_exposure).collect();

    let range = |values: &[f64]| {
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let padding = (max - min).abs().max(1.0) * 0.1;
        (min - padding)..(max + padding)
    };
    // Exposure is a bounded domain (see EXPOSURE_MIN/MAX) — clip the x-axis
    // to it exactly rather than padding past 0/100.
    let (x_range, y_range) = (EXPOSURE_MIN..EXPOSURE_MAX, range(&yields));
    let count_range = 0.0..(quotes.len() as f64);

    // Step curve for the running count of quotes at or below each exposure:
    // flat, then a vertical jump at each sorted exposure value.
    let mut sorted_exposures = exposures.clone();
    sorted_exposures.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut count_steps = Vec::with_capacity(sorted_exposures.len() * 2 + 2);
    count_steps.push((x_range.start, 0.0));
    for (i, &e) in sorted_exposures.iter().enumerate() {
        count_steps.push((e, i as f64));
        count_steps.push((e, (i + 1) as f64));
    }
    count_steps.push((x_range.end, sorted_exposures.len() as f64));

    let root = BitMapBackend::new(path, (960, 540)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Target Exposure vs Expected Yield", ("sans-serif", 28))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(50)
        .right_y_label_area_size(50)
        .build_cartesian_2d(x_range.clone(), y_range.clone())?
        .set_secondary_coord(x_range, count_range);

    chart
        .configure_mesh()
        .x_desc("target exposure")
        .y_desc("expected yield")
        .draw()?;

    chart
        .configure_secondary_axes()
        .y_desc("quotes at/below exposure")
        .draw()?;

    // Tranche bands, shaded by rate, drawn before the scatter so points sit
    // on top.
    let min_rate = tranches
        .iter()
        .map(|t| t.rate)
        .fold(f64::INFINITY, f64::min);
    let max_rate = tranches
        .iter()
        .map(|t| t.rate)
        .fold(f64::NEG_INFINITY, f64::max);
    let rate_span = (max_rate - min_rate).max(f64::EPSILON);

    for t in tranches {
        let intensity = (t.rate - min_rate) / rate_span; // 0 (cheapest) .. 1 (priciest)
        let band_color = RGBColor(
            (255.0 - 80.0 * intensity) as u8,
            (220.0 - 140.0 * intensity) as u8,
            (120.0 + 40.0 * intensity) as u8,
        )
        .mix(0.35);

        chart.draw_series(std::iter::once(Rectangle::new(
            [(t.attachment, y_range.start), (t.detachment, y_range.end)],
            band_color.filled(),
        )))?;

        let label_x = (t.attachment + t.detachment) / 2.0;
        let label_y = y_range.end - (y_range.end - y_range.start) * 0.05;
        chart.draw_series(std::iter::once(Text::new(
            format!("{:.2}", t.rate),
            (label_x, label_y),
            ("sans-serif", 16).into_font().color(&BLACK),
        )))?;
    }

    // The noiseless curve quotes are actually sampled around, before jitter.
    chart
        .draw_series(LineSeries::new(
            generation_curve(200),
            ShapeStyle::from(&BLACK).stroke_width(2),
        ))?
        .label("generation curve")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK.stroke_width(2)));

    let used: Vec<_> = tranches.iter().flat_map(|t| t.sample.iter()).collect();
    const USED_GREEN: RGBColor = RGBColor(34, 139, 34);

    chart
        .draw_series(quotes.iter().map(|q| {
            let color = if used.iter().any(|&u| u == q) {
                USED_GREEN
            } else {
                RED
            };
            // Bubble radius scales with order size (mean 1.0 -> radius 4).
            let radius = (4.0 * q.size).clamp(2.0, 12.0) as i32;
            Circle::new((q.target_exposure, q.expected_yield), radius, color.filled())
        }))?
        .label("quote")
        .legend(|(x, y)| Circle::new((x, y), 4, RED.filled()));

    chart
        .draw_series(std::iter::empty::<Circle<(f64, f64), i32>>())?
        .label("quote used in tranche rate")
        .legend(|(x, y)| Circle::new((x, y), 4, USED_GREEN.filled()));

    // Step line tracing each tranche's rate across its exposure band.
    let rate_steps: Vec<(f64, f64)> = tranches
        .iter()
        .flat_map(|t| [(t.attachment, t.rate), (t.detachment, t.rate)])
        .collect();
    chart
        .draw_series(LineSeries::new(rate_steps, &MAGENTA))?
        .label("tranche rate")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

    chart
        .draw_secondary_series(LineSeries::new(count_steps, &BLUE))?
        .label("cumulative count")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
