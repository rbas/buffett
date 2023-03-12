pub mod dummy;
pub mod error;

use crate::entity::{Currency, Ticker, TrashHoldConf};

use self::error::FetchError;

pub trait TrashHoldConfRepository {
    fn get_conf_for(
        &self,
        ticker: Ticker,
        greather_than: Currency,
        less_than: Currency,
    ) -> Result<Vec<TrashHoldConf>, FetchError>;
}
