use chrono::NaiveDate;
use config::{load_config, Config};
use entity::Ticker;
use repository::{
    pushover::PushOverStockEventRepository, sqlite::SqliteStockTrashHoldRepository,
    StockEventRepository, StockTrashHoldRepository,
};
use rusqlite::Connection;

use crate::data_collector::download_data;

mod config;
mod data_collector;
mod entity;
mod repository;

fn build_trash_hold_repository(config: &Config) -> Box<dyn StockTrashHoldRepository> {
    let connection = Connection::open(&config.db.path).unwrap();

    Box::new(SqliteStockTrashHoldRepository::new(connection))
}

fn build_stock_event_repository(config: &Config) -> Box<dyn StockEventRepository> {
    Box::new(PushOverStockEventRepository::new(
        config.pushover.api_url.to_owned(),
        config.pushover.app_token.to_owned(),
        config.pushover.delivery_group_token.to_owned(),
    ))
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let tickers = vec![
        "0777.HK", "INFU", "0468.HK", "1126.HK", "CZO.V", "SMSI", "CE1.AX", "CEPU", "RSSS",
        "LMN.SW", "TPCS", "PXT.TO", "CNE.TO", "AMBP", "MBR.WA",
    ];
    let market_data_date = NaiveDate::from_ymd_opt(2022, 12, 9).unwrap();

    let mut tasks = Vec::with_capacity(tickers.len());

    for ticker in tickers {
        tasks.push(tokio::spawn(download_data(
            Ticker::from(ticker),
            market_data_date,
        )));
    }

    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        outputs.push(task.await.unwrap());
    }
    println!("{:?}", outputs);

    let ticker = Ticker::from("SMSI");
    let current_value = 1.190;

    let config = load_config("./conf.toml");

    let trash_hold_repository = build_trash_hold_repository(&config);
    let event_reposiotry = build_stock_event_repository(&config);

    let entities = match trash_hold_repository.get_stock_trash_hold_for(ticker, current_value) {
        Ok(entities) => entities,
        Err(error) => {
            panic!("{:#?}", error)
        }
    };

    for trash_hold in &entities {
        match event_reposiotry.register_changes(trash_hold.ticker.clone(), current_value) {
            Ok(entity) => {
                println!("{:#?}", entity);
            }
            Err(err) => println!("{:?}", err),
        }
    }
    println!("{:#?}", entities);
}
