pub fn read_player_index() {
    let df_csv = CsvReadOptions::default()
        .with_infer_schema_length(None)
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("../../../assets/data/output.csv".into()))?
        .finish()?;
    println!("{}", df_csv);
}

#[cfg(test)]
mod tests {
    // use super::*;
    use polars::prelude::*;

    #[test]
    fn test_read_player_index() {
        let path = r"C:\Users\Danie\Desktop\programming\rust\nba_prediction_engine\data\prepared_data\fetched_PlayerIndex_data.csv";

        let player_index = CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(path.into()))
            .unwrap()
            .finish()
            .unwrap();

        println!("{}", player_index);

        let path = r"C:\Users\Danie\Desktop\programming\rust\nba_prediction_engine\data\prepared_data\fetched_PlayerGeneralAverages_data.csv";

        let player_averages = CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(path.into()))
            .unwrap()
            .finish()
            .unwrap();

        println!("{}", player_averages);

        let path = r"C:\Users\Danie\Desktop\programming\rust\nba_prediction_engine\data\fetched_data_output_as_string\fetched_InjuryReport_data";

        let mut file = std::fs::File::open(path).unwrap();
        let player_injury_report = JsonReader::new(&mut file).finish().unwrap();

        println!("{}", player_injury_report);
    }
}
