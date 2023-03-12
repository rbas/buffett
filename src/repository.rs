pub mod dummy;
pub mod error;

use crate::entity::{Currency, Ticker, TrashHoldConf};

use self::error::FetchError;

pub trait TrashHoldConfRepository {
    fn get_conf_for(
        &self,
        ticker: Ticker,
        value: Currency,
    ) -> Result<Vec<TrashHoldConf>, FetchError>;
}
