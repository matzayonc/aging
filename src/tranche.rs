use crate::data_consumer::QuoteHistory;
use crate::data_producer::{EXPOSURE_MAX, EXPOSURE_MIN, Quote};

/// A reinsurance-style layer of the exposure axis: covers quotes whose
/// `target_exposure` falls in `[attachment, detachment)`, priced at `rate`.
#[derive(Debug, Clone, PartialEq)]
pub struct TrancheOrder {
    pub attachment: f64,
    pub detachment: f64,
    pub rate: f64,
    /// The (up to `RATE_SAMPLE_SIZE`) cheapest quotes actually used to
    /// compute `rate`.
    pub sample: Vec<Quote>,
}

/// Nominal band edges as fractions of the exposure axis, mortgage-style:
/// a fat bottom tranche (the safe 50%), then progressively smaller,
/// riskier tranches toward the top — on round numbers (50/25/15/5/5). The
/// former top tranche (90-100) is now split in half (90-95/95-100).
const BAND_FRACTIONS: [f64; 6] = [0.0, 0.5, 0.75, 0.9, 0.95, 1.0];

/// A 25-wide band wants at least this much total order size claimed
/// before it's considered "filled".
const BASE_MIN_SIZE: f64 = 10.0;
const BASE_WIDTH: f64 = 25.0;

/// A band's rate is priced off (at most) this many of its cheapest
/// claimed orders.
const RATE_SAMPLE_SIZE: usize = 5;

/// How much total order size a band of the given nominal width should
/// require — scaled down for narrower bands so a thin tranche (e.g. a
/// 5-wide slice) doesn't have to raid most of the axis just to fill a
/// quota sized for a much wider band.
fn min_size_for_width(width: f64) -> f64 {
    (BASE_MIN_SIZE / BASE_WIDTH * width).max(0.05)
}

/// Splits the exposure axis into tranche bands (per `BAND_FRACTIONS`)
/// computed from `history`'s quotes, covering the full
/// `[EXPOSURE_MIN, EXPOSURE_MAX]` range with no gaps or overlaps.
///
/// Bands start at their nominal (fixed) width. A band short on claimed
/// order size widens rightward, absorbing not-yet-claimed quotes in
/// exposure order, until its claimed quotes' sizes sum to at least
/// `min_size_for_width` of its *nominal* width (or it runs out of quotes)
/// — each quote is claimed by exactly one band. A band's rate is the mean
/// `expected_yield` of the (up to `RATE_SAMPLE_SIZE`) cheapest quotes it
/// claimed.
pub fn compute_tranches(history: &QuoteHistory) -> Vec<TrancheOrder> {
    let mut quotes: Vec<_> = history.quotes.iter().collect();
    quotes.sort_by(|a, b| a.target_exposure.partial_cmp(&b.target_exposure).unwrap());

    let span = EXPOSURE_MAX - EXPOSURE_MIN;
    let edges: Vec<f64> = BAND_FRACTIONS
        .iter()
        .map(|f| EXPOSURE_MIN + f * span)
        .collect();
    let band_count = edges.len() - 1;

    let mut tranches = Vec::with_capacity(band_count);
    let mut attachment = EXPOSURE_MIN;
    let mut claimed = 0usize;

    for band in 0..band_count {
        let is_last = band == band_count - 1;
        let size_cap = min_size_for_width(edges[band + 1] - edges[band]);

        let mut end = claimed;
        let mut detachment = if is_last {
            EXPOSURE_MAX
        } else {
            edges[band + 1].max(attachment)
        };
        let mut claimed_size = 0.0;

        // Claim everything already within the nominal band.
        while end < quotes.len() && quotes[end].target_exposure < detachment {
            claimed_size += quotes[end].size;
            end += 1;
        }
        // Short on size: widen rightward until we have enough, or run out.
        if !is_last {
            while claimed_size < size_cap && end < quotes.len() {
                claimed_size += quotes[end].size;
                detachment = quotes[end].target_exposure.max(detachment);
                end += 1;
            }
        } else {
            end = quotes.len();
        }

        let claimed_quotes = &quotes[claimed..end];
        let mut by_yield: Vec<_> = claimed_quotes.to_vec();
        by_yield.sort_by(|a, b| a.expected_yield.partial_cmp(&b.expected_yield).unwrap());
        let sample = &by_yield[..by_yield.len().min(RATE_SAMPLE_SIZE)];
        let rate = if sample.is_empty() {
            0.0
        } else {
            sample.iter().map(|q| q.expected_yield).sum::<f64>() / sample.len() as f64
        };

        tranches.push(TrancheOrder {
            attachment,
            detachment,
            rate,
            sample: sample.iter().map(|&&q| q).collect(),
        });
        attachment = detachment;
        claimed = end;
    }

    tranches
}
