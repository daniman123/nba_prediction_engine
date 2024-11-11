#![allow(dead_code)]

use std::fs::File;

use crate::{ error::FetchError, fetch_data, Endpoint };

use csv::Writer;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct TodaysScoreboardFetchedResponse {
    meta: Value,
    scoreboard: TodaysScoreboardData,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct TodaysScoreboardData {
    gameDate: String,
    leagueId: String,
    leagueName: String,
    games: Vec<Value>,
}

pub async fn fetch_todays_scoreboard() -> Result<(), FetchError> {
    // API Endpoint
    let endpoint = Endpoint::TodaysScoreboard.url();
    // File Name for future reference
    let file_name = Endpoint::TodaysScoreboard.file_name();

    // Fetch Data
    let data: TodaysScoreboardFetchedResponse = fetch_data(endpoint, file_name).await?;

    let file = File::create("")?;
    let mut wtr = Writer::from_writer(file);

    let scoreboard = &data.scoreboard;
    let games = &scoreboard.games;

    let headers = vec![
        "away_teamName",
        "away_teamCity",
        "away_teamId",
        "away_score",
        "home_teamName",
        "home_teamCity",
        "home_teamId",
        "home_score"
    ];

    wtr.write_record(headers)?;

    for game in games {
        let away_team = game.get("awayTeam").unwrap();
        let away_team_name = away_team
            .get("teamName")
            .unwrap()
            .clone()
            .as_str()
            .unwrap()
            .to_string();
        let away_team_city = away_team
            .get("teamCity")
            .unwrap()
            .clone()
            .as_str()
            .unwrap()
            .to_string();
        let away_team_id = away_team.get("teamId").unwrap().to_string();
        let away_team_score = away_team.get("score").unwrap().to_string();

        let home_team = game.get("homeTeam").unwrap();
        let home_team_name = home_team
            .get("teamName")
            .unwrap()
            .clone()
            .as_str()
            .unwrap()
            .to_string();
        let home_team_city = home_team
            .get("teamCity")
            .unwrap()
            .clone()
            .as_str()
            .unwrap()
            .to_string();
        let home_team_id = home_team.get("teamId").unwrap().to_string();
        let home_team_score = home_team.get("score").unwrap().to_string();

        let game_record = vec![
            away_team_name,
            away_team_city,
            away_team_id,
            away_team_score,
            home_team_name,
            home_team_city,
            home_team_id,
            home_team_score
        ];

        wtr.write_record(game_record)?;
    }

    wtr.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_name() {
        let file = fs::File
            ::open("../data/seed_data/todaysScoreboard_00.json")
            .expect("file should open read only");
        let json: TodaysScoreboardFetchedResponse = serde_json
            ::from_reader(file)
            .expect("file should be proper JSON");
        let scoreboard = &json.scoreboard;
        let games = &scoreboard.games;

        let mut csv_record_collection: Vec<Vec<String>> = vec![];

        for game in games {
            let away_team = game.get("awayTeam").unwrap();
            let away_team_name = away_team
                .get("teamName")
                .unwrap()
                .clone()
                .as_str()
                .unwrap()
                .to_string();
            let away_team_city = away_team
                .get("teamCity")
                .unwrap()
                .clone()
                .as_str()
                .unwrap()
                .to_string();
            let away_team_id = away_team.get("teamId").unwrap().to_string();
            let away_team_score = away_team.get("score").unwrap().to_string();

            let home_team = game.get("homeTeam").unwrap();
            let home_team_name = home_team
                .get("teamName")
                .unwrap()
                .clone()
                .as_str()
                .unwrap()
                .to_string();
            let home_team_city = home_team
                .get("teamCity")
                .unwrap()
                .clone()
                .as_str()
                .unwrap()
                .to_string();
            let home_team_id = home_team.get("teamId").unwrap().to_string();
            let home_team_score = home_team.get("score").unwrap().to_string();

            let game_record = vec![
                away_team_name,
                away_team_city,
                away_team_id,
                away_team_score,
                home_team_name,
                home_team_city,
                home_team_id,
                home_team_score
            ];

            csv_record_collection.push(game_record);
        }

        println!("{:#?}", csv_record_collection)
    }
}
