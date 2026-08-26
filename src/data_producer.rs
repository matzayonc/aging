/// A quote produced by a `DataProducer`: the yield expected on the
/// position, the exposure being targeted for it, and the order's size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quote {
    pub expected_yield: f64,
    pub target_exposure: f64,
    pub size: f64,
}

/// Produces pricing data: an expected yield and a target exposure.
pub trait DataProducer {
    fn produce(&self) -> Quote;
}

pub(crate) const YIELD_MIN: f64 = 3.0;
pub(crate) const YIELD_MAX: f64 = 10.0;
pub(crate) const EXPOSURE_MIN: f64 = 0.0;
pub(crate) const EXPOSURE_MAX: f64 = 100.0;

/// Order size is bell-curved around one unit, with a standard deviation
/// of 20% of that unit.
const SIZE_MEAN: f64 = 1.0;
const SIZE_STD_DEV: f64 = 0.2;

/// Standard normal sample via the Box-Muller transform, so we don't need a
/// distributions crate just for this.
fn standard_normal(rng: &mut rand::rngs::StdRng) -> f64 {
    use rand::RngExt;
    let u1: f64 = rng.random::<f64>().max(f64::MIN_POSITIVE); // avoid ln(0)
    let u2: f64 = rng.random();
    (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
}

/// Monotonic cubic (t³): maps t ∈ [0, 1] to [0, 1], the shape of the
/// exposure → yield correlation below. Flat near the low end, gentle
/// through the middle, then its slope triples by t = 1 — yield barely
/// rises until exposure is past ~90, then turns sharply upward.
fn yield_curve(t: f64) -> f64 {
    t * t * t
}

/// The noiseless exposure/yield curve quotes are sampled around (see
/// `SimulatedDataProducer::produce`): `steps + 1` points from
/// `t = 0` to `t = 1`, before any jitter is applied.
pub fn generation_curve(steps: usize) -> Vec<(f64, f64)> {
    (0..=steps)
        .map(|i| {
            let t = i as f64 / steps as f64;
            let exposure = EXPOSURE_MIN + t * (EXPOSURE_MAX - EXPOSURE_MIN);
            let yield_ = YIELD_MIN + yield_curve(t) * (YIELD_MAX - YIELD_MIN);
            (exposure, yield_)
        })
        .collect()
}

/// A demo producer that samples points along a nonlinear (cubic)
/// yield/exposure curve and jitters each variable independently, standing
/// in until a real data source exists.
///
/// Seeded, so a given seed always reproduces the same sequence of quotes.
pub struct SimulatedDataProducer {
    pub yield_noise: f64,
    pub exposure_noise: f64,
    rng: std::cell::RefCell<rand::rngs::StdRng>,
}

impl SimulatedDataProducer {
    pub fn new(yield_noise: f64, exposure_noise: f64, seed: u64) -> Self {
        use rand::SeedableRng;
        Self {
            yield_noise,
            exposure_noise,
            rng: std::cell::RefCell::new(rand::rngs::StdRng::seed_from_u64(seed)),
        }
    }
}

impl DataProducer for SimulatedDataProducer {
    fn produce(&self) -> Quote {
        use rand::RngExt;
        let mut rng = self.rng.borrow_mut();

        // Shared position along the curve: this is what creates the
        // correlation between the two outputs.
        let t: f64 = rng.random();
        let base_exposure = EXPOSURE_MIN + t * (EXPOSURE_MAX - EXPOSURE_MIN);
        let base_yield = YIELD_MIN + yield_curve(t) * (YIELD_MAX - YIELD_MIN);

        // Independent jitter: separate draws, separate magnitudes.
        let yield_jitter = (rng.random::<f64>() - 0.5) * 2.0 * self.yield_noise;
        let exposure_jitter = (rng.random::<f64>() - 0.5) * 2.0 * self.exposure_noise;
        let size = (SIZE_MEAN + SIZE_STD_DEV * standard_normal(&mut rng)).max(0.05);

        Quote {
            expected_yield: (base_yield + yield_jitter).clamp(YIELD_MIN, YIELD_MAX),
            target_exposure: (base_exposure + exposure_jitter).clamp(EXPOSURE_MIN, EXPOSURE_MAX),
            size,
        }
    }
}
