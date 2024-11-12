use crate::{
    error::FetchError,
    fetch_data,
    result_sets::{write_vector_to_csv, ResultSetsFetchedResponse},
    Endpoint,
};

pub async fn fetch_teams_general_opponent() -> Result<(), FetchError> {
    // API Endpoint
    let endpoint = Endpoint::TeamsGeneralOpponent.url();
    // File Name for future reference
    let file_name = Endpoint::TeamsGeneralOpponent.file_name();
    let save_file_dir = Endpoint::prepared_data_file_path();
    let mut save_file_path = save_file_dir.join(file_name);
    save_file_path.set_extension("csv");
    // Fetch Data
    let data: ResultSetsFetchedResponse = fetch_data(endpoint, file_name).await?;
    let headers = &data.resultSets[0].headers;
    let rows = &data.resultSets[0].rowSet;
    write_vector_to_csv(Some(headers.to_vec()), rows.to_vec(), save_file_path).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, path::Path};

    use super::*;

    #[test]
    fn test_name() {
        let file_name = Endpoint::TeamsGeneralOpponent.file_name();
        // let save_file_dir = Endpoint::prepared_data_file_path();

        let binding = env::current_dir().unwrap();
        let cwd = binding.parent().unwrap();
        println!("cwd: {:?}", cwd);
        let file_path = Path::new(&cwd).join("data").join("prepared_data");

        let mut save_file_path = file_path.join(file_name);
        save_file_path.set_extension("csv");

        println!("save_file_path: {:?}", save_file_path);

        // Check if the file exists
        if file_path.exists() {
            println!("File exists at {:?}", file_path);
        } else {
            println!("File does not exist at {:?}", file_path);
        }
    }
}
