use crate::{
    error::FetchError,
    fetch_data,
    result_sets::{write_vector_to_csv, ResultSetsFetchedResponse},
    Endpoint,
};

pub async fn fetch_player_general_averages() -> Result<(), FetchError> {
    // API Endpoint
    let endpoint = Endpoint::PlayerGeneralAverages.url();
    // File Name for future reference
    let file_name = Endpoint::PlayerGeneralAverages.file_name();

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
