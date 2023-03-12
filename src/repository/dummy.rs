use crate::{
    entity::{Currency, Ticker, TrashHoldConf},
    repository::TrashHoldConfRepository,
};

use super::error::FetchError;

pub struct DummyTrashHoldConfRepository {}

impl TrashHoldConfRepository for DummyTrashHoldConfRepository {
    fn get_conf_for(
        &self,
        ticker: Ticker,
        value: Currency,
    ) -> Result<Vec<TrashHoldConf>, FetchError> {
        let mut entities: Vec<TrashHoldConf> = Vec::new();

        entities.push(TrashHoldConf {
            ticker: ticker,
            greather_than: value - 1.0,
            less_than: value + 2.0,
        });

        Ok(entities)
    }
}
