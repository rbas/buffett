use std::ops::Add;

use chrono::{Duration, NaiveDate};
use reqwest;

#[derive(Debug)]
pub enum DowloaderError {
    BadRequest,
    FetchData,
    UnexpectedHttpStatus,
}

pub async fn fetch_data(url: &str) -> Result<String, DowloaderError> {
    let response = reqwest::get(url).await;
    if let Err(_err) = &response {
        return Err(DowloaderError::BadRequest);
    }

    let body = response.unwrap();
    match body.status() {
        reqwest::StatusCode::OK => match body.text().await {
            Err(_err) => Err(DowloaderError::FetchData),
            Ok(text) => Ok(text),
        },
        _ => Err(DowloaderError::UnexpectedHttpStatus),
    }
}

pub fn url_builder(ticker: &str, market_data_date: NaiveDate) -> String {
    let market_data_date = market_data_date.and_hms_opt(00, 00, 00).unwrap();

    let time_from = market_data_date.timestamp();
    let time_to = market_data_date.add(Duration::days(1)).timestamp();

    format!(
        "https://query1.finance.yahoo.com/v7/finance/download/{ticker}?period1={time_from}&period2={time_to}&interval=1d&events=history&includeAdjustedClose=true",
        ticker = ticker,
        time_from = time_from,
        time_to = time_to
    )
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn build_url_for_smsi() {
        let date = NaiveDate::from_ymd_opt(2022, 12, 9).unwrap();

        let exptected_url = "https://query1.finance.yahoo.com/v7/finance/download/SMSI?period1=1670544000&period2=1670630400&interval=1d&events=history&includeAdjustedClose=true";

        let built_url = url_builder("SMSI", date);

        assert_eq!(exptected_url, built_url);
    }
}
