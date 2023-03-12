pub type Ticker = String;
pub type Currency = u32;

pub struct TrashHoldConf {
    ticker: Ticker,
    greather_than: Currency,
    less_than: Currency,
}
