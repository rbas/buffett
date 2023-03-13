use async_trait::async_trait;

use crate::{
    entity::{Currency, StockEvent, StockTrashHold, Ticker},
    repository::StockTrashHoldRepository,
};

use super::{
    error::{FetchError, SaveError},
    StockEventRepository,
};

pub struct DummyStockTrashHoldRepository {}
pub struct DummyStockEventRepository {}

#[async_trait]
impl StockTrashHoldRepository for DummyStockTrashHoldRepository {
    async fn get_stock_trash_hold_for(
        &self,
        ticker: Ticker,
        value: Currency,
    ) -> Result<Vec<StockTrashHold>, FetchError> {
        let entities: Vec<StockTrashHold> = vec![StockTrashHold {
            ticker,
            greather_than: value - 1.0,
            less_than: value + 2.0,
        }];

        Ok(entities)
    }
}

#[async_trait]
impl StockEventRepository for DummyStockEventRepository {
    async fn register_changes(
        &self,
        ticker: Ticker,
        value: Currency,
    ) -> Result<StockEvent, SaveError> {
        let event = StockEvent { ticker, value };

        Ok(event)
    }
}
