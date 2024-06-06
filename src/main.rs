#![allow(clippy::needless_return)]

use anyhow::anyhow;
use chrono::{DateTime, NaiveTime, TimeDelta, Utc};
use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TrackingEntry {
    id: u32,
    workspace_id: u32,
    start: DateTime<Utc>,
    duration: i64,
}

#[derive(Deserialize, Debug)]
struct Workspace {
    id: u32,
}

#[derive(Parser)]
struct CliArgs {
    #[command(subcommand)]
    command: Option<CliCommand>,
}

#[derive(Subcommand)]
enum CliCommand {
    Status,
    Stop,
    Start,
}

fn start(client: &reqwest::blocking::Client, token: &str) {
    let workspaces = client
        .get("https://api.track.toggl.com/api/v9/workspaces")
        .basic_auth(token, Some("api_token"))
        .send()
        .unwrap()
        .json::<Vec<Workspace>>()
        .unwrap();

    client
        .post(format!(
            "https://api.track.toggl.com/api/v9/workspaces/{}/time_entries",
            workspaces[0].id
        ))
        .basic_auth(token, Some("api_token"))
        .json(&serde_json::json!({
            "start": chrono::Utc::now().to_rfc3339(),
            "created_with": "tracker CLI",
            "workspace_id": workspaces[0].id,
            "duration": -1,
        }))
        .send()
        .unwrap();
}

fn stop(client: &reqwest::blocking::Client, token: &str, current_entry: &TrackingEntry) {
    client
        .patch(format!(
            "https://api.track.toggl.com/api/v9/workspaces/{}/time_entries/{}/stop",
            current_entry.workspace_id, current_entry.id
        ))
        .basic_auth(token, Some("api_token"))
        .send()
        .unwrap();
}

fn status(client: &reqwest::blocking::Client, token: &str) -> Option<TrackingEntry> {
    let response = client
        .get("https://api.track.toggl.com/api/v9/me/time_entries/current")
        .basic_auth(token, Some("api_token"))
        .send()
        .unwrap();

    if response.content_length().unwrap() != 4 {
        // assuming response is not "null", stopping current entry
        return Some(response.json::<TrackingEntry>().unwrap());
    } else {
        // assuming response is "null", starting new entry
        return None;
    }
}

fn format_duration_human(duration: &TimeDelta) -> String {
    let mut components = Vec::new();

    let seconds = duration.num_seconds();
    if seconds > 3600 {
        components.push(format!("{} hour(s)", seconds / 3600));
    }

    let seconds = seconds % 3600;
    if seconds > 60 {
        components.push(format!("{} minute(s)", seconds / 60));
    }

    let seconds = seconds % 60;
    if seconds > 0 {
        components.push(format!("{} second(s)", seconds));
    }

    if components.len() > 1 {
        return format!(
            "{} and {}",
            components[0..components.len() - 1].join(", "),
            components[components.len() - 1]
        );
    }
    return components.last().unwrap().to_string();
}

fn get_entries(
    client: &reqwest::blocking::Client,
    token: &str,
    start_date: chrono::DateTime<Utc>,
    end_date: chrono::DateTime<Utc>,
) -> Vec<TrackingEntry> {
    return client
        .get("https://api.track.toggl.com/api/v9/me/time_entries")
        .basic_auth(token, Some("api_token"))
        .query(&[
            ("start_date", start_date.to_rfc3339()),
            ("end_date", end_date.to_rfc3339()),
        ])
        .send()
        .unwrap()
        .json::<Vec<TrackingEntry>>()
        .unwrap();
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let token = std::env::var("TOGGL_TRACK_TOKEN").map_err(|_| {
        anyhow!("Please set TOGGL_TRACK_TOKEN environment variable with the API token")
    })?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );
    let client = reqwest::blocking::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    match args.command {
        Some(CliCommand::Status) => {
            match status(&client, &token) {
                Some(entry) => {
                    println!(
                        "current: {}",
                        format_duration_human(&(Utc::now() - entry.start))
                    );
                }
                None => println!("current: stopped"),
            }
            let total_duration = get_entries(
                &client,
                &token,
                Utc::now()
                    .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                    .unwrap(),
                Utc::now(),
            )
            .iter()
            .map(|entry| {
                if entry.duration == -1 {
                    (Utc::now() - entry.start).num_seconds()
                } else {
                    entry.duration
                }
            })
            .sum::<i64>();
            println!(
                "today: {}",
                format_duration_human(&TimeDelta::seconds(total_duration))
            );
        }
        Some(CliCommand::Stop) => match status(&client, &token) {
            Some(entry) => {
                stop(&client, &token, &entry);
                println!("stopped");
            }
            None => println!("already stopped"),
        },
        Some(CliCommand::Start) => match status(&client, &token) {
            Some(_) => {
                println!("already running");
            }
            None => {
                start(&client, &token);
                println!("started");
            }
        },
        None => match status(&client, &token) {
            Some(current_entry) => {
                stop(&client, &token, &current_entry);
                println!("stopped");
            }
            None => {
                start(&client, &token);
                println!("started");
            }
        },
    }

    Ok(())
}
