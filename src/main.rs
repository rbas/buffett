use entity::{Currency, Ticker};
use repository::{dummy::DummyTrashHoldConfRepository, TrashHoldConfRepository};

mod entity;
mod repository;

fn main() {
    let repository = DummyTrashHoldConfRepository {};

    match repository.get_conf_for(Ticker::from("APPL"), Currency::from(18.4)) {
        Ok(entities) => {
            println!("{:#?}", entities)
        }
        Err(error) => {
            println!("{:#?}", error)
        }
    }
}
