pub mod dummy;
pub mod error;

use crate::entity::{Currency, StockTrashHold, Ticker};

use self::error::FetchError;

pub trait StockTrashHoldRepository {
    fn get_conf_for(
        &self,
        ticker: Ticker,
        value: Currency,
    ) -> Result<Vec<StockTrashHold>, FetchError>;
}
