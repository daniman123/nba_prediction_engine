use deunicode::deunicode;
use polars::prelude::*;

pub fn read_player_index() -> DataFrame {
    let path = r"C:\Users\Danie\Desktop\programming\rust\nba_prediction_engine\data\prepared_data\fetched_PlayerIndex_data.csv";

    // Load the CSV as a LazyFrame
    let player_index_df = LazyCsvReader::new(path)
        .with_has_header(true)
        .finish()
        .unwrap();

    let mut player_index_df = player_index_df
        .select([
            col("*"),
            concat_str(
                [col("PLAYER_FIRST_NAME"), col("PLAYER_LAST_NAME")],
                " ",
                false,
            )
            .alias("PLAYER_NAME"),
        ])
        .collect()
        .unwrap();

    // Define columns to drop
    let col_drop_player_index_df = [
        "HEIGHT",
        "WEIGHT",
        "COLLEGE",
        "COUNTRY",
        "DRAFT_YEAR",
        "DRAFT_ROUND",
        "DRAFT_NUMBER",
        "ROSTER_STATUS",
        "FROM_YEAR",
        "TO_YEAR",
        "PLAYER_SLUG",
        "IS_DEFUNCT",
        "STATS_TIMEFRAME",
        "JERSEY_NUMBER",
        "PTS",
        "REB",
        "AST",
        "TEAM_SLUG",
    ];

    // Apply transformations in lazy mode
    player_index_df = player_index_df.drop_many(col_drop_player_index_df);

    let player_index_df = player_index_df
        .apply("PLAYER_NAME", normalize_name)
        .unwrap();
    player_index_df.to_owned()
}

// Function to transliterate non-English Latin characters to ASCII
fn normalize_name(player_name: &Column) -> Column {
    player_name
        .str()
        .unwrap()
        .into_iter()
        .map(|opt_name: Option<&str>| opt_name.map(|name: &str| deunicode(name)))
        .collect::<StringChunked>()
        .into_column()
}

pub fn read_player_averages() -> DataFrame {
    let path = r"C:\Users\Danie\Desktop\programming\rust\nba_prediction_engine\data\prepared_data\fetched_PlayerGeneralAverages_data.csv";

    let player_averages_df = CsvReadOptions::default()
        .with_infer_schema_length(None)
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(path.into()))
        .unwrap()
        .finish()
        .unwrap();

    let col_drop_player_averages_df = [
        "TEAM_ID",
        "TEAM_ABBREVIATION",
        "NICKNAME",
        "PLAYER_NAME",
        "AGE",
        "W",
        "L",
        "W_PCT",
        "PLUS_MINUS",
        "NBA_FANTASY_PTS",
        "DD2",
        "TD3",
        "WNBA_FANTASY_PTS",
        "GP_RANK",
        "W_RANK",
        "L_RANK",
        "W_PCT_RANK",
        "MIN_RANK",
        "FGM_RANK",
        "FGA_RANK",
        "FG_PCT_RANK",
        "FG3M_RANK",
        "FG3A_RANK",
        "FG3_PCT_RANK",
        "FTM_RANK",
        "FTA_RANK",
        "FT_PCT_RANK",
        "OREB_RANK",
        "DREB_RANK",
        "REB_RANK",
        "AST_RANK",
        "TOV_RANK",
        "STL_RANK",
        "BLK_RANK",
        "BLKA_RANK",
        "PF_RANK",
        "PFD_RANK",
        "PTS_RANK",
        "PLUS_MINUS_RANK",
        "NBA_FANTASY_PTS_RANK",
        "DD2_RANK",
        "TD3_RANK",
        "WNBA_FANTASY_PTS_RANK",
    ];

    player_averages_df.drop_many(col_drop_player_averages_df)
}

pub fn read_player_injury_report() -> DataFrame {
    let path = r"C:\Users\Danie\Desktop\programming\rust\nba_prediction_engine\data\fetched_data_output_as_string\fetched_InjuryReport_data";

    let mut file = std::fs::File::open(path).unwrap();
    let player_injury_report_df = JsonReader::new(&mut file).finish().unwrap();
    let col_drop_player_injury_report_df = [
        "ID",
        "URL",
        "firstname",
        "lastname",
        "team",
        "position",
        "rDate",
    ];

    player_injury_report_df.drop_many(col_drop_player_injury_report_df)
}

pub fn player_stats_df() {
    let player_index_df = read_player_index();

    let player_averages_df = read_player_averages();

    let player_index_averages_df = player_index_df
        .left_join(&player_averages_df, ["PERSON_ID"], ["PLAYER_ID"])
        .unwrap();

    let player_injury_report_df = read_player_injury_report();

    let mut player_index_averages_injury_report_df = player_index_averages_df
        .left_join(&player_injury_report_df, ["PLAYER_NAME"], ["player"])
        .unwrap();

    let path = r"C:\Users\Danie\Desktop\programming\rust\nba_prediction_engine\data\wrangled_data";
    let mut file = std::fs::File::create(path).unwrap();
    CsvWriter::new(&mut file)
        .finish(&mut player_index_averages_injury_report_df)
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_player_data() {
        let player_index = read_player_index();
        println!("{}", player_index);

        let player_averages = read_player_averages();
        println!("{}", player_averages);

        let player_injury_report = read_player_injury_report();
        println!("{}", player_injury_report);
    }

    #[test]
    fn test_remove_player_data_cols() {
        let player_index_df = read_player_index();
        let col_drop_player_index_df = [
            "HEIGHT",
            "WEIGHT",
            "COLLEGE",
            "COUNTRY",
            "DRAFT_YEAR",
            "DRAFT_ROUND",
            "DRAFT_NUMBER",
            "ROSTER_STATUS",
            "FROM_YEAR",
            "TO_YEAR",
            "PLAYER_SLUG",
            "IS_DEFUNCT",
            "STATS_TIMEFRAME",
            "JERSEY_NUMBER",
            "PTS",
            "REB",
            "AST",
            "TEAM_SLUG",
        ];
        let player_index_df = player_index_df.drop_many(col_drop_player_index_df);
        let cols_player_index = player_index_df.get_column_names();
        println!("cols_player_index: {:#?}", cols_player_index);

        let player_averages_df = read_player_averages();
        let col_drop_player_averages_df = [
            "TEAM_ID",
            "TEAM_ABBREVIATION",
            "NICKNAME",
            "AGE",
            "W",
            "L",
            "W_PCT",
            "PLUS_MINUS",
            "NBA_FANTASY_PTS",
            "DD2",
            "TD3",
            "WNBA_FANTASY_PTS",
            "GP_RANK",
            "W_RANK",
            "L_RANK",
            "W_PCT_RANK",
            "MIN_RANK",
            "FGM_RANK",
            "FGA_RANK",
            "FG_PCT_RANK",
            "FG3M_RANK",
            "FG3A_RANK",
            "FG3_PCT_RANK",
            "FTM_RANK",
            "FTA_RANK",
            "FT_PCT_RANK",
            "OREB_RANK",
            "DREB_RANK",
            "REB_RANK",
            "AST_RANK",
            "TOV_RANK",
            "STL_RANK",
            "BLK_RANK",
            "BLKA_RANK",
            "PF_RANK",
            "PFD_RANK",
            "PTS_RANK",
            "PLUS_MINUS_RANK",
            "NBA_FANTASY_PTS_RANK",
            "DD2_RANK",
            "TD3_RANK",
            "WNBA_FANTASY_PTS_RANK",
        ];
        let player_averages_df = player_averages_df.drop_many(col_drop_player_averages_df);
        let cols_player_averages = player_averages_df.get_column_names();
        println!("cols_player_averages: {:#?}", cols_player_averages);

        let player_injury_report_df = read_player_injury_report();
        let col_drop_player_injury_report_df = [
            "ID",
            "URL",
            "firstname",
            "lastname",
            "team",
            "position",
            "rDate",
        ];
        let player_injury_report_df =
            player_injury_report_df.drop_many(col_drop_player_injury_report_df);
        let cols_player_injury_report = player_injury_report_df.get_column_names();
        println!(
            "cols_player_injury_report: {:#?}",
            cols_player_injury_report
        );
    }

    #[test]
    fn test_collect_player_data_dfs() {
        let player_index_df = read_player_index();

        let player_averages_df = read_player_averages();

        let player_index_averages_df = player_index_df
            .left_join(&player_averages_df, ["PERSON_ID"], ["PLAYER_ID"])
            .unwrap();

        let player_injury_report_df = read_player_injury_report();

        let mut player_index_averages_injury_report_df = player_index_averages_df
            .left_join(&player_injury_report_df, ["PLAYER_NAME"], ["player"])
            .unwrap();

        let mut file = std::fs::File::create("player_index_averages_injury_report_df.csv").unwrap();
        CsvWriter::new(&mut file)
            .finish(&mut player_index_averages_injury_report_df)
            .unwrap();
    }
}
