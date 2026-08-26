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

/// Tranche boundaries as fractions of *total accepted liquidity*
/// (`sum of every quote's size`), mortgage-style: a fat bottom tranche
/// (the safe 50% of capital), then progressively smaller, riskier
/// tranches toward the top (50/25/15/5/5). There's no absolute/global
/// cap — the boundary nominally at fraction `f` always has exactly `f` of
/// total liquidity below it (protecting it) and `1 - f` above it (at risk
/// first, in a loss), no matter how much total liquidity there actually
/// is.
const BAND_FRACTIONS: [f64; 6] = [0.0, 0.5, 0.75, 0.9, 0.95, 1.0];

/// A band's rate is priced off (at most) this many of its cheapest
/// claimed orders.
const RATE_SAMPLE_SIZE: usize = 5;

/// Splits the exposure axis into tranche bands (per `BAND_FRACTIONS`)
/// computed from `history`'s quotes, covering the full
/// `[EXPOSURE_MIN, EXPOSURE_MAX]` range with no gaps or overlaps.
///
/// A single forward pass over quotes sorted by `target_exposure`: walk
/// cumulative `size` and cut a band as soon as that running total crosses
/// its fraction of total liquidity. Because the fractions sum to 1.0 and
/// liquidity is monotonic, every band lands on (essentially) exactly its
/// target share — no widening or shortfall handling needed. A band's rate
/// is the mean `expected_yield` of the (up to `RATE_SAMPLE_SIZE`)
/// cheapest quotes it claimed.
pub fn compute_tranches(history: &QuoteHistory) -> Vec<TrancheOrder> {
    let mut quotes: Vec<_> = history.quotes.iter().collect();
    quotes.sort_by(|a, b| a.target_exposure.partial_cmp(&b.target_exposure).unwrap());

    let total_liquidity: f64 = quotes.iter().map(|q| q.size).sum();
    let liquidity_edges: Vec<f64> = BAND_FRACTIONS.iter().map(|f| f * total_liquidity).collect();
    let band_count = liquidity_edges.len() - 1;

    let mut tranches = Vec::with_capacity(band_count);
    let mut attachment = EXPOSURE_MIN;
    let mut claimed = 0usize;
    let mut cumulative_size = 0.0;

    for band in 0..band_count {
        let is_last = band == band_count - 1;
        let liquidity_target = liquidity_edges[band + 1];

        let mut end = claimed;
        if is_last {
            end = quotes.len();
        } else {
            while cumulative_size < liquidity_target && end < quotes.len() {
                cumulative_size += quotes[end].size;
                end += 1;
            }
        }

        let detachment = if end < quotes.len() {
            quotes[end].target_exposure
        } else {
            EXPOSURE_MAX
        };

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
