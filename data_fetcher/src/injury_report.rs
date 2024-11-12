use crate::{error::FetchError, fetch_data, Endpoint};
use std::{collections::HashSet, fs::File, path::PathBuf};

use csv::Writer;
use serde_json::Value;

type InjuryReportFetchedResponse = Vec<Value>;

fn json_array_to_csv(data: Vec<Value>, file_path: PathBuf) -> Result<(), FetchError> {
    let mut headers = HashSet::new();
    for obj in &data {
        if let Value::Object(map) = obj {
            for key in map.keys() {
                headers.insert(key.clone());
            }
        }
    }

    
    let file = File::create(file_path)?;
    let mut wtr = Writer::from_writer(file);
    
    let headers: Vec<&str> = headers.iter().map(AsRef::as_ref).collect();
    wtr.write_record(&headers)?;

    for obj in data {
        if let Value::Object(map) = obj {
            let row: Vec<String> = headers
                .iter()
                .map(|key| {
                    map.get(*key)
                        .map_or("".to_string(), |v| v.as_str().unwrap().to_string())
                })
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
    let save_file_dir = Endpoint::prepared_data_file_path();
    let mut save_file_path = save_file_dir.join(file_name);
    save_file_path.set_extension("csv");
    // Fetch Data
    let data: InjuryReportFetchedResponse = fetch_data(endpoint, file_name).await?;
    json_array_to_csv(data, save_file_path)?;
    Ok(())
}
