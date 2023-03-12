use config::{load_config, Config};
use entity::{Currency, Ticker};
use repository::{
    pushover::PushOverStockEventRepository, sqlite::SqliteStockTrashHoldRepository,
    StockEventRepository, StockTrashHoldRepository,
};
use rusqlite::Connection;

mod config;
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

fn main() {
    let ticker = Ticker::from("SMSI");
    let current_value = Currency::from(1.190);

    let config = load_config(&"./conf.toml");

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
