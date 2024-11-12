use crate::error::FetchError;

use csv::Writer;
use serde::Deserialize;
use serde_json::Value;
use std::{fs::File, path::PathBuf};

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
    pub rowSet: Vec<Vec<Value>>,
}

pub fn write_vector_to_csv(
    headers: Option<Vec<String>>,
    rows: Vec<Vec<Value>>,
    path: PathBuf,
) -> Result<(), FetchError> {
    let file = File::create(path)?;
    let mut wtr = Writer::from_writer(file);

    if let Some(columns) = headers {
        wtr.write_record(&columns)?;
    }

    for row in rows {
        let string_row: Vec<String> = row
            .iter()
            .map(|value| {
                match value {
                    Value::String(s) => s.clone(),     // Extract the inner String as-is
                    Value::Number(n) => n.to_string(), // Convert Number to String
                    Value::Bool(b) => b.to_string(),   // Convert Boolean to String
                    Value::Null => "".to_string(),     // Handle Null as empty string
                    _ => "".to_string(),               // Handle other types if necessary
                }
            })
            .collect();
        wtr.write_record(&string_row)?;
    }
    wtr.flush()?;
    Ok(())
}
