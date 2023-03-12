use std::error::Error;

use super::DataRecord;

pub fn parse_market_data_from_csv(csv_data: &str) -> Result<Option<DataRecord>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let mut iter = reader.deserialize();

    match iter.next() {
        Some(result) => {
            let record: DataRecord = result?;
            Ok(Some(record))
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn parse_data_from_csv() {
        let csv_data = "Date,Open,High,Low,Close,Adj Close,Volume
        2022-12-06,2.190000,2.190000,2.110000,2.130000,2.130000,106000";

        let expected_date = NaiveDate::from_ymd_opt(2022, 12, 6).unwrap();
        let expected_data = DataRecord {
            date: expected_date,
            open: 2.19,
            high: 2.19,
            low: 2.11,
            close: 2.13,
        };

        match parse_market_data_from_csv(csv_data) {
            Ok(Some(market_data)) => assert_eq!(expected_data, market_data),
            Ok(None) => {
                println!("Expected some data structure");
                assert!(false)
            }
            Err(error) => {
                println!("{:?}", error);
                assert!(false)
            }
        }
    }
}
