use crate::error::FetchError;

use std::fs::File;
use serde::Deserialize;
use serde_json::Value;
use csv::Writer;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultSetsFetchedResponse {
    pub resource: Value,
    pub parameters: Value,
    pub resultSets: Vec<ResultSetsData>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultSetsData {
    pub name: String,
    pub headers: Vec<String>,
    pub rowSets: Vec<Vec<Value>>,
}

pub fn write_vector_to_csv(
    headers: Option<Vec<String>>,
    rows: Vec<Vec<Value>>,
    path: &str
) -> Result<(), FetchError> {
    let file = File::create(path)?;
    let mut wtr = Writer::from_writer(file);

    if let Some(columns) = headers {
        wtr.write_record(&columns)?;
    }

    for row in rows {
        let string_row: Vec<String> = row
            .iter()
            .map(|value| value.to_string())
            .collect();
        wtr.write_record(&string_row)?;
    }
    wtr.flush()?;
    Ok(())
}
