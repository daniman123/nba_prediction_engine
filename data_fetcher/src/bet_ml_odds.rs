use csv::Writer;
use serde_json::Value;

use crate::{error::FetchError, fetch_data, Endpoint};
use std::fs::File;

pub async fn fetch_bet_ml_odds() -> Result<(), FetchError> {
    // API Endpoint
    let endpoint = Endpoint::Bet365Odds.url();
    // File Name for future reference
    let file_name = Endpoint::Bet365Odds.file_name();

    // Fetch Data
    let data: Value = fetch_data(endpoint, file_name).await?;

    let game_odds = data.get("pageProps").unwrap().get("oddsTables").unwrap()[0]
        .get("oddsTableModel")
        .unwrap()
        .get("gameRows")
        .unwrap()
        .as_array()
        .unwrap();

    let save_file_dir = Endpoint::prepared_data_file_path();
    let mut save_file_path = save_file_dir.join(file_name);
    save_file_path.set_extension("csv");

    let file = File::create(save_file_path)?;
    let mut wtr = Writer::from_writer(file);

    for game in game_odds {
        let current_line = game.get("oddsViews").unwrap()[3]
            .get("currentLine")
            .unwrap();
        let opening_line = game.get("oddsViews").unwrap()[3]
            .get("openingLine")
            .unwrap();

        let away_odds = current_line.get("awayOdds").unwrap().to_string();
        let open_away_odds = opening_line.get("awayOdds").unwrap().to_string();
        let away_team_name = game
            .get("gameView")
            .unwrap()
            .get("awayTeam")
            .unwrap()
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let home_odds = current_line.get("homeOdds").unwrap().to_string();
        let open_home_odds = opening_line.get("homeOdds").unwrap().to_string();
        let home_team_name = game
            .get("gameView")
            .unwrap()
            .get("homeTeam")
            .unwrap()
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        let ml_odds = vec![
            away_odds,
            open_away_odds,
            away_team_name,
            home_odds,
            open_home_odds,
            home_team_name,
        ];

        wtr.write_record(ml_odds)?;
    }

    wtr.flush()?;
    Ok(())
}
