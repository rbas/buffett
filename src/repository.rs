use crate::entity::{Currency, Ticker, TrashHoldConf};

pub enum FetchError {}

pub trait TrashHoldConfRepository {
    fn get_conf_for(
        &self,
        ticker: Ticker,
        greather_than: Currency,
        less_than: Currency,
    ) -> Result<Vec<TrashHoldConf>, FetchError>;
}
