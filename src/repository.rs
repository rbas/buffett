pub mod dummy;
pub mod error;

use crate::entity::{Currency, StockEvent, StockTrashHold, Ticker};

use self::error::{FetchError, SaveError};

pub trait StockTrashHoldRepository {
    fn get_conf_for(
        &self,
        ticker: Ticker,
        value: Currency,
    ) -> Result<Vec<StockTrashHold>, FetchError>;
}

pub trait StockEventRepository {
    fn register_changes(&self, ticker: Ticker, value: Currency) -> Result<StockEvent, SaveError>;
}
