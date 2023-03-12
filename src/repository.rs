pub mod dummy;
pub mod error;
pub mod pushover;

use crate::entity::{Currency, StockEvent, StockTrashHold, Ticker};

use self::error::{FetchError, SaveError};

pub trait StockTrashHoldRepository {
    fn get_stock_trash_hold_for(
        &self,
        ticker: Ticker,
        value: Currency,
    ) -> Result<Vec<StockTrashHold>, FetchError>;
}

pub trait StockEventRepository {
    fn register_changes(&self, ticker: Ticker, value: Currency) -> Result<StockEvent, SaveError>;
}
