pub mod bet_ml_odds;
pub mod error;
pub mod injury_report;
pub mod opponent_shooting_general;
pub mod player_general_averages;
pub mod player_index;
pub mod result_sets;
pub mod season_schedule;
pub mod teams_general_advanced;
pub mod teams_general_opponent;
// pub mod todays_scoreboard;

use crate::error::FetchError;
use bet_ml_odds::fetch_bet_ml_odds;
use flate2::read::GzDecoder;
use injury_report::fetch_injury_report;
use opponent_shooting_general::fetch_opponent_shooting_general;
use player_general_averages::fetch_player_general_averages;
use player_index::fetch_player_index;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use season_schedule::fetch_season_schedule;
use serde::de::DeserializeOwned;
use std::io::Read;
use std::io::{self, Write};
use std::path::Path;
use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};
use teams_general_advanced::fetch_teams_general_advanced;
use teams_general_opponent::fetch_teams_general_opponent;
use tracing::{debug, error, info};

#[derive(Debug)]
pub enum Endpoint {
    SeasonSchedule,
    TodaysScoreboard,
    PlayerIndex,
    PlayerGeneralAverages,
    TeamsGeneralAdvanced,
    OpponentShootingGeneral,
    TeamsGeneralOpponent,
    InjuryReport,
    Bet365Odds,
}

impl Endpoint {
    pub fn url(&self) -> &'static str {
        match self {
            Endpoint::SeasonSchedule =>
                "https://cdn.nba.com/static/json/staticData/scheduleLeagueV2_51.json",
            Endpoint::TodaysScoreboard =>
                "https://cdn.nba.com/static/json/liveData/scoreboard/todaysScoreboard_00.json",
            Endpoint::PlayerIndex =>
                "https://stats.nba.com/stats/playerindex?College=&Country=&DraftPick=&DraftRound=&DraftYear=&Height=&Historical=0&LeagueID=00&Season=2024-25&SeasonType=Regular%20Season&TeamID=0&Weight=",
            Endpoint::PlayerGeneralAverages =>
                "https://stats.nba.com/stats/leaguedashplayerstats?College=&Conference=&Country=&DateFrom=&DateTo=&Division=&DraftPick=&DraftYear=&GameScope=&GameSegment=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2024-25&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&StarterBench=&TeamID=0&VsConference=&VsDivision=&Weight=",
            Endpoint::TeamsGeneralAdvanced =>
                "https://stats.nba.com/stats/leaguedashteamstats?Conference=&DateFrom=&DateTo=&Division=&GameScope=&GameSegment=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&MeasureType=Advanced&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2024-25&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&StarterBench=&TeamID=0&TwoWay=0&VsConference=&VsDivision=",
            Endpoint::OpponentShootingGeneral =>
                "https://stats.nba.com/stats/leaguedashoppptshot?Conference=&DateFrom=&DateTo=&Division=&GameSegment=&GeneralRange=Overall&LastNGames=0&LeagueID=00&Location=&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&Season=2024-25&SeasonSegment=&SeasonType=Regular%20Season&TeamID=0&VsConference=&VsDivision=",
            Endpoint::TeamsGeneralOpponent =>
                "https://stats.nba.com/stats/leaguedashteamstats?Conference=&DateFrom=&DateTo=&Division=&GameScope=&GameSegment=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2024-25&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&StarterBench=&TeamID=0&TwoWay=0&VsConference=&VsDivision=",
            Endpoint::InjuryReport =>
                "https://www.rotowire.com/basketball/tables/injury-report.php?team=ALL&pos=ALL",
            Endpoint::Bet365Odds =>
                "https://www.sportsbookreview.com/_next/data/z65WyQ2UMs83FwnSdRynY/betting-odds/nba-basketball/money-line/full-game.json?league=nba-basketball&oddsType=money-line&oddsScope=full-game",
        }
    }

    pub fn file_name(&self) -> &'static str {
        match self {
            Endpoint::SeasonSchedule => "fetched_SeasonSchedule_data",
            Endpoint::TodaysScoreboard => "fetched_TodaysScoreboard_data",
            Endpoint::PlayerIndex => "fetched_PlayerIndex_data",
            Endpoint::PlayerGeneralAverages => "fetched_PlayerGeneralAverages_data",
            Endpoint::TeamsGeneralAdvanced => "fetched_TeamsGeneralAdvanced_data",
            Endpoint::OpponentShootingGeneral => "fetched_OpponentShootingGeneral_data",
            Endpoint::TeamsGeneralOpponent => "fetched_TeamsGeneralOpponent_data",
            Endpoint::InjuryReport => "fetched_InjuryReport_data",
            Endpoint::Bet365Odds => "fetched_Bet365Odds_data",
        }
    }
    pub fn prepared_data_file_path() -> PathBuf {
        let parent = env::current_dir().unwrap();
        let cwd = parent.parent().unwrap();
        let file_path = Path::new(&cwd).join("data").join("prepared_data");
        // "../data/prepared_data"
        file_path
    }
}

pub async fn fetch_data<T>(url: &str, file_name: &str) -> Result<T, FetchError>
where
    T: DeserializeOwned,
{
    let client = Client::new();
    let headers = build_headers();

    debug!("Attempting to fetch data from: {}", url);

    let response = client.get(url).headers(headers).send().await?;

    if !response.status().is_success() {
        error!("Unexpected status code: {}", response.status());
        return Err(FetchError::UnexpectedStatusCode(response.status()));
    }

    let bytes = response.bytes().await?;
    let data = decompress_or_convert(&bytes, file_name)?;

    info!("Successfully fetched and parsed data from: {}", url);
    Ok(serde_json::from_str(&data)?)
}

fn write_to_file(path: &str, filename: &str, content: &str) -> io::Result<()> {
    fs::create_dir_all(path)?; // Create the directory if it doesn't exist
    let mut file_path = Path::new(path).join(filename);
    file_path.set_extension("txt");
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn build_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers.insert(
        "Accept",
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    headers.insert("x-nba-stats-token", HeaderValue::from_static("true"));
    headers.insert(
        "User-Agent",
        HeaderValue::from_static("Mozilla/5.0 (compatible; MyApp/1.0)"),
    );
    headers.insert("x-nba-stats-origin", HeaderValue::from_static("stats"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert(
        "Referer",
        HeaderValue::from_static("https://stats.nba.com/"),
    );
    headers.insert(
        "Accept-Encoding",
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert(
        "Accept-Language",
        HeaderValue::from_static("en-US,en;q=0.9"),
    );
    headers
}

fn decompress_or_convert(bytes: &[u8], file_name: &str) -> Result<String, FetchError> {
    let mut decoder = GzDecoder::new(bytes);
    let mut decompressed_data = String::new();

    match decoder.read_to_string(&mut decompressed_data) {
        Ok(_) => {
            write_to_file(
                "../data/fetched_data_output_as_string",
                file_name,
                &decompressed_data,
            )?;
            Ok(decompressed_data)
        }
        Err(_) => {
            // If decompression fails, try converting bytes directly to a UTF-8 string
            let converted_data = String::from_utf8(bytes.to_vec()).map_err(FetchError::Utf8)?;
            write_to_file(
                "../data/fetched_data_output_as_string",
                file_name,
                &converted_data,
            )?;
            Ok(converted_data)
        }
    }
}

pub async fn gather_and_prepare_fetched_data() {
    fetch_teams_general_opponent().await.unwrap();
    fetch_teams_general_advanced().await.unwrap();
    fetch_season_schedule().await.unwrap();
    fetch_player_index().await.unwrap();
    fetch_player_general_averages().await.unwrap();
    fetch_opponent_shooting_general().await.unwrap();
    fetch_injury_report().await.unwrap();
    fetch_bet_ml_odds().await.unwrap();
}
