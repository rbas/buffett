use config::load_config;
use entity::{Currency, Ticker};
use repository::{
    dummy::DummyStockTrashHoldRepository, pushover::PushOverStockEventRepository,
    StockEventRepository, StockTrashHoldRepository,
};

mod config;
mod entity;
mod repository;

fn main() {
    let trash_hold_repository = DummyStockTrashHoldRepository {};

    let config = load_config(&"./conf.toml");

    let event_reposiotry = PushOverStockEventRepository::new(
        config.pushover.api_url,
        config.pushover.app_token,
        config.pushover.delivery_group_token,
    );

    let ticker = Ticker::from("SMSI");
    let current_value = Currency::from(135.4);

    match trash_hold_repository.get_stock_trash_hold_for(ticker, current_value) {
        Ok(entities) => {
            for trash_hold in &entities {
                event_reposiotry
                    .register_changes(trash_hold.ticker.clone(), current_value)
                    .unwrap();
            }
            println!("{:#?}", entities);
        }
        Err(error) => {
            println!("{:#?}", error)
        }
    }
}
