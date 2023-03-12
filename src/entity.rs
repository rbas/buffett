pub type Ticker = String;
pub type Currency = f32;

#[derive(Debug)]
pub struct TrashHoldConf {
    pub ticker: Ticker,
    pub greather_than: Currency,
    pub less_than: Currency,
}
