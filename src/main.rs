use entity::{Currency, Ticker};
use repository::{dummy::DummyStockTrashHoldRepository, StockTrashHoldRepository};

mod entity;
mod repository;

fn main() {
    let repository = DummyStockTrashHoldRepository {};

    match repository.get_conf_for(Ticker::from("APPL"), Currency::from(18.4)) {
        Ok(entities) => {
            println!("{:#?}", entities)
        }
        Err(error) => {
            println!("{:#?}", error)
        }
    }
}
