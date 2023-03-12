use crate::entity::{Currency, StockEvent, Ticker};

use super::{error::SaveError, StockEventRepository};

pub struct PushOverStockEventRepository {
    api_url: String,
    app_token: String,
    delivery_group_token: String,
}

impl PushOverStockEventRepository {
    pub fn new(api_url: String, app_token: String, delivery_group_token: String) -> Self {
        PushOverStockEventRepository {
            api_url,
            app_token,
            delivery_group_token,
        }
    }
}

impl StockEventRepository for PushOverStockEventRepository {
    fn register_changes(&self, ticker: Ticker, value: Currency) -> Result<StockEvent, SaveError> {
        let client = reqwest::blocking::Client::new();

        let title = format!("{} update", ticker);
        let message = format!("Ticker {} has change value to {}", ticker, value);

        let params = [
            ("user", &self.delivery_group_token),
            ("token", &self.app_token),
            ("title", &title),
            ("message", &message),
        ];
        match client.post(&self.api_url).form(&params).send() {
            Ok(response) => {
                let se = StockEvent { ticker, value };
                println!(
                    "Status code {}\nBody {}\n",
                    response.status(),
                    response.text().unwrap()
                );
                Ok(se)
            }
            Err(err) => {
                println!("{:#?}", err);

                Err(SaveError {})
            }
        }
    }
}
