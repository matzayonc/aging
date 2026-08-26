use crate::data_producer::Quote;

/// Consumes quotes as they're produced.
pub trait DataConsumer {
    fn consume(&mut self, quote: Quote);
}

/// A consumer that just keeps every quote it's given, in order.
#[derive(Debug, Default)]
pub struct QuoteHistory {
    pub quotes: Vec<Quote>,
}

impl DataConsumer for QuoteHistory {
    fn consume(&mut self, quote: Quote) {
        self.quotes.push(quote);
    }
}
