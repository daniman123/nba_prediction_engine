use crate::{
    error::FetchError,
    fetch_data,
    result_sets::{ write_vector_to_csv, ResultSetsFetchedResponse },
    Endpoint,
};

pub async fn fetch_player_index() -> Result<(), FetchError> {
    // API Endpoint
    let endpoint = Endpoint::PlayerIndex.url();
    // File Name for future reference
    let file_name = Endpoint::PlayerIndex.file_name();

    // Fetch Data
    let data: ResultSetsFetchedResponse = fetch_data(endpoint, file_name).await?;
    let headers = &data.resultSets[0].headers;
    let rows = &data.resultSets[0].rowSets;
    write_vector_to_csv(Some(headers.to_vec()), rows.to_vec(), "").unwrap();
    Ok(())
}
