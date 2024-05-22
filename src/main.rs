use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TrackingEntry {
    id: u32,
    workspace_id: u32,
}

#[derive(Deserialize, Debug)]
struct Workspace {
    id: u32,
}

fn main() {
    let token = std::env::var("TOGGLE_TRACK_TOKEN").unwrap();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );
    let client = reqwest::blocking::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    let response = client
        .get("https://api.track.toggl.com/api/v9/me/time_entries/current")
        .basic_auth(&token, Some("api_token"))
        .send()
        .unwrap();

    if response.content_length().unwrap() != 4 {
        // assuming response is not "null", stopping current entry
        let current_entry = response.json::<TrackingEntry>().unwrap();
        client
            .patch(format!(
                "https://api.track.toggl.com/api/v9/workspaces/{}/time_entries/{}/stop",
                current_entry.workspace_id, current_entry.id
            ))
            .basic_auth(&token, Some("api_token"))
            .send()
            .unwrap();
        println!("stopped");
    } else {
        // assuming response is "null", starting new entry
        let workspaces = client
            .get("https://api.track.toggl.com/api/v9/workspaces")
            .basic_auth(&token, Some("api_token"))
            .send()
            .unwrap()
            .json::<Vec<Workspace>>()
            .unwrap();

        client
            .post(format!(
                "https://api.track.toggl.com/api/v9/workspaces/{}/time_entries",
                workspaces[0].id
            ))
            .basic_auth(&token, Some("api_token"))
            // '{"created_with":"API example code","description":"Hello Toggl","tags":[],"billable":false,"workspace_id":{workspace_id},"duration":-1,"start":"1984-06-08T11:02:53.000Z","stop":null}'
            .json(&serde_json::json!({
                "start": chrono::Utc::now().to_rfc3339(),
                "created_with": "tracker CLI",
                "workspace_id": workspaces[0].id,
                "duration": -1,
            }))
            .send()
            .unwrap();
        println!("started");
    }
}
