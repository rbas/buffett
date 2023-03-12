use entity::{Currency, Ticker};
use repository::{
    dummy::{DummyStockEventRepository, DummyStockTrashHoldRepository},
    StockEventRepository, StockTrashHoldRepository,
};

mod entity;
mod repository;

fn main() {
    let trash_hold_repository = DummyStockTrashHoldRepository {};
    let event_reposiotry = DummyStockEventRepository {};

    let ticker = Ticker::from("APPL");
    let current_value = Currency::from(18.4);
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
