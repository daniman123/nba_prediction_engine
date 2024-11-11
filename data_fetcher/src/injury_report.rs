use std::{ collections::HashSet, fs::File };

use csv::Writer;
use serde_json::Value;
use crate::{ error::FetchError, fetch_data, Endpoint };

type InjuryReportFetchedResponse = Vec<Value>;

fn json_array_to_csv(data: Vec<Value>, file_path: &str) -> Result<(), FetchError> {
    let mut headers = HashSet::new();
    for obj in &data {
        if let Value::Object(map) = obj {
            for key in map.keys() {
                headers.insert(key.clone());
            }
        }
    }

    let headers: Vec<&str> = headers.iter().map(AsRef::as_ref).collect();

    let file = File::create(file_path)?;
    let mut wtr = Writer::from_writer(file);

    for obj in data {
        if let Value::Object(map) = obj {
            let row: Vec<String> = headers
                .iter()
                .map(|key| { map.get(*key).map_or("".to_string(), |v| v.to_string()) })
                .collect();
            wtr.write_record(&row)?;
        }
    }
    wtr.flush()?;
    Ok(())
}

pub async fn fetch_injury_report() -> Result<(), FetchError> {
    // API Endpoint
    let endpoint = Endpoint::InjuryReport.url();
    // File Name for future reference
    let file_name = Endpoint::InjuryReport.file_name();

    // Fetch Data
    let data: InjuryReportFetchedResponse = fetch_data(endpoint, file_name).await?;
    json_array_to_csv(data, "")?;
    Ok(())
}
