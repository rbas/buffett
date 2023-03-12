pub type Ticker = String;
pub type Currency = f32;

#[derive(Debug)]
pub struct StockTrashHold {
    pub ticker: Ticker,
    pub greather_than: Currency,
    pub less_than: Currency,
}

#[derive(Debug)]
pub struct StockEvent {
    pub ticker: Ticker,
    pub value: Currency,
}

#[derive(Debug)]
pub struct Notification {
    pub title: String,
    pub message: String,
}
