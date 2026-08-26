mod data_consumer;
mod data_producer;
mod graph;
mod tranche;

use data_consumer::{DataConsumer, QuoteHistory};
use data_producer::{DataProducer, SimulatedDataProducer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let producer = SimulatedDataProducer::new(0.35, 5.0, 42);
    let mut history = QuoteHistory::default();

    for _ in 0..100 {
        history.consume(producer.produce());
    }

    let tranches = tranche::compute_tranches(&history);

    let out_path = "expectations.png";
    graph::plot_expectations(&history, &tranches, out_path)?;
    println!("wrote {out_path} from {} quotes", history.quotes.len());

    Ok(())
}
