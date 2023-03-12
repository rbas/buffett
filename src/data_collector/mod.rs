use chrono::NaiveDate;
use log::{error, info};
use serde::Deserialize;

use crate::data_collector::{
    downloader::{fetch_data, url_builder},
    parser::parse_market_data_from_csv,
};

pub type Currency = f32;
pub type Ticker = String;

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataRecord {
    date: NaiveDate,
    open: Currency,
    high: Currency,
    low: Currency,
    close: Currency,
}

#[derive(Debug, Deserialize)]
pub struct MarketData {
    pub ticker: Ticker,
    pub data: Vec<DataRecord>,
}

pub mod downloader;
pub mod parser;

pub async fn download_data(ticker: Ticker, market_data_date: NaiveDate) -> Option<MarketData> {
    let url = url_builder(&ticker, market_data_date);
    let body = fetch_data(&url).await;

    info!(
        "[{ticker}] dowloading market data for {ticker} from date {date}",
        ticker = ticker,
        date = market_data_date
    );
    match body {
        Ok(csv) => match parse_market_data_from_csv(&csv) {
            Ok(data) => {
                if data.is_some() {
                    Some(MarketData {
                        ticker: Ticker::from(ticker),
                        data: vec![data.unwrap()],
                    })
                } else {
                    None
                }
            }
            Err(err) => {
                error!(
                    "[{}] During parsing market data occured error: {}",
                    ticker, err
                );
                None
            }
        },
        Err(err) => {
            error!("[{}] Data could not be downlaoded: Err{:?}", ticker, err);
            None
        }
    }
}
