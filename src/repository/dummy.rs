use crate::{
    entity::{Currency, StockTrashHold, Ticker},
    repository::StockTrashHoldRepository,
};

use super::error::FetchError;

pub struct DummyStockTrashHoldRepository {}

impl StockTrashHoldRepository for DummyStockTrashHoldRepository {
    fn get_conf_for(
        &self,
        ticker: Ticker,
        value: Currency,
    ) -> Result<Vec<StockTrashHold>, FetchError> {
        let mut entities: Vec<StockTrashHold> = Vec::new();

        entities.push(StockTrashHold {
            ticker: ticker,
            greather_than: value - 1.0,
            less_than: value + 2.0,
        });

        Ok(entities)
    }
}
