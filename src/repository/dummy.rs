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
        greather_than: Currency,
        less_than: Currency,
    ) -> Result<Vec<TrashHoldConf>, FetchError> {
        let mut entities: Vec<TrashHoldConf> = Vec::new();

        entities.push(TrashHoldConf {
            ticker: ticker,
            greather_than: greather_than - 1.0,
            less_than: less_than + 2.0,
        });

        Ok(entities)
    }
}
