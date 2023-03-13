use async_trait::async_trait;
use sqlx::{Error, SqlitePool};

use crate::entity::{StockTrashHold, Ticker};

use super::{error::FetchError, StockTrashHoldRepository};

impl From<Error> for FetchError {
    fn from(err: Error) -> FetchError {
        FetchError::new(format!("{:?}", err))
    }
}

#[derive(Debug, sqlx::FromRow)]
struct StockTrashHoldRecord {
    id: i64,
    ticker: String,
    greather_than: f32,
    less_than: f32,
}

impl StockTrashHoldRecord {
    fn to_entity(&self) -> StockTrashHold {
        StockTrashHold {
            ticker: Ticker::from(&self.ticker),
            greather_than: self.greather_than,
            less_than: self.less_than,
        }
    }
}

pub struct SqliteStockTrashHoldRepository {
    connection: SqlitePool,
}

impl SqliteStockTrashHoldRepository {
    pub fn new(connection: SqlitePool) -> Self {
        SqliteStockTrashHoldRepository { connection }
    }
}

#[async_trait]
impl StockTrashHoldRepository for SqliteStockTrashHoldRepository {
    async fn get_stock_trash_hold_for(
        &self,
        ticker: crate::entity::Ticker,
        value: crate::entity::Currency,
    ) -> Result<Vec<crate::entity::StockTrashHold>, super::error::FetchError> {
        let mut conn = self.connection.acquire().await?;

        let stream = sqlx::query_as::<_, StockTrashHoldRecord>(
            "SELECT id, ticker, greather_than, less_than 
                FROM stock_trash_hold 
                WHERE ticker=? 
                AND (greather_than < ?) or (less_than > ?)",
        )
        .bind(ticker)
        .bind(value)
        .bind(value)
        .fetch_all(&mut conn)
        .await?;

        let entities = stream.iter().map(|r| r.to_entity()).collect();

        Ok(entities)
    }
}
