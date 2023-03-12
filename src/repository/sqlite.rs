use rusqlite::{params, Connection, Error};

use crate::entity::{Currency, StockTrashHold, Ticker};

use super::{error::FetchError, StockTrashHoldRepository};

impl From<Error> for FetchError {
    fn from(err: Error) -> FetchError {
        FetchError::new(format!("{:?}", err))
    }
}

#[derive(Debug)]
struct StockTrashHoldRecord {
    id: i32,
    ticker: String,
    greather_than: f32,
    less_than: f32,
}

impl StockTrashHoldRecord {
    fn to_entity(&self) -> StockTrashHold {
        StockTrashHold {
            ticker: Ticker::from(&self.ticker),
            greather_than: Currency::from(self.greather_than),
            less_than: Currency::from(self.less_than),
        }
    }
}

pub struct SqliteStockTrashHoldRepository {
    connection: Connection,
}

impl SqliteStockTrashHoldRepository {
    pub fn new(connection: Connection) -> Self {
        SqliteStockTrashHoldRepository {
            connection: connection,
        }
    }
}

impl StockTrashHoldRepository for SqliteStockTrashHoldRepository {
    fn get_stock_trash_hold_for(
        &self,
        ticker: crate::entity::Ticker,
        value: crate::entity::Currency,
    ) -> Result<Vec<crate::entity::StockTrashHold>, super::error::FetchError> {
        let params = params![ticker, value, value];
        let mut statemant = self.connection.prepare("SELECT id, ticker, greather_than, less_than FROM stock_trash_hold WHERE ticker=?1 AND (greather_than < ?2) or (less_than > ?3)")?;

        let rows = statemant.query_map(params, |row| {
            Ok(StockTrashHoldRecord {
                id: row.get(0)?,
                ticker: row.get(1)?,
                greather_than: row.get(2)?,
                less_than: row.get(3)?,
            })
        })?;

        let mut entities = Vec::new();
        for record in rows {
            entities.push(record?.to_entity());
        }

        Ok(entities)
    }
}