use std::{ fs::File, io::{ BufWriter, Write } };

use crate::{ error::FetchError, fetch_data, Endpoint };

use serde::{ Deserialize, Serialize };
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case, unused)]
struct LeagueScheduleFetchedResponse {
    meta: Value,
    leagueSchedule: LeagueScheduleData,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case, unused)]
struct LeagueScheduleData {
    seasonYear: String,
    leagueId: String,
    gameDates: Vec<GameDatesData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct GameDatesData {
    gameDate: String,
    games: Vec<Value>,
}

pub async fn fetch_season_schedule() -> Result<(), FetchError> {
    // API Endpoint
    let endpoint = Endpoint::SeasonSchedule.url();
    // File Name for future reference
    let file_name = Endpoint::SeasonSchedule.file_name();

    // Fetch Data
    let data: LeagueScheduleFetchedResponse = fetch_data(endpoint, file_name).await?;

    let game_dates = data.leagueSchedule.gameDates;

    let file = File::create("")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &game_dates)?;
    writer.flush()?;
    Ok(())
}
