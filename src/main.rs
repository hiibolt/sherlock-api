use std::process::Stdio;

use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message}, response::{IntoResponse, Response}, routing::{get, post}, Router
};
use anyhow::{ Result, anyhow, Context };
use tokio::{io::{AsyncBufReadExt, BufReader}, process::Command};

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}
async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let username = if let Ok(Message::Text(username)) = msg {
            username
        } else {
            // Client disconnected
            return;
        };

        println!("Received websocket request for '{username}'...");

        let proxy_link = std::env::var("PROXY_LINK").expect("Missing PROXY_LINK env variable!");
        let invalid_sites: [&str; 39] = [
            "Oracle", "8tracks", "Coders Rank", "Fiverr",
            "HackerNews", "Modelhub", "metacritic", "xHamster",
            "CNET", "YandexMusic", "HackerEarth", "OpenStreetMap", 
            "Pinkbike", "Slides", "Strava", "Archive", "CGTrader",
            "G2G", "NationStates", "IFTTT", "SoylentNews", "hunting",
            "Contently", "Euw", "OurDJTalk", "BitCoinForum", "HEXRPG",
            "Polymart", "Linktree", "GeeksforGeeks", "Kongregate", "RedTube",
            "APClips", "Heavy-R", "RocketTube", "Zhihu", "NitroType", "babyRU",
            "freecodecamp"
        ];

        let mut cmd = Command::new("python")
            .arg("sherlock/sherlock")
            .arg("--nsfw")
            .arg("--folderoutput")
            .arg("sherlock_output")
            .arg("--timeout")
            .arg("5")
            .arg("--proxy")
            .arg(proxy_link)
            .arg("--local")
            .arg(&format!("{username}"))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Issue running the Sherlock command! Did you install with Nix?");
        {
            let stdout = cmd.stdout.as_mut().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let mut stdout_lines = stdout_reader.lines();

            let mut found = false;
            while let Ok(Some(output)) = stdout_lines.next_line().await {
                if invalid_sites
                        .iter()
                        .any(|site| output.contains(site))
                {
                    continue;
                }

                if output.contains("http") || output.contains("https") {
                    println!("Found site for {username}: {output}");

                    found = true;

                    if socket.send(Message::Text(format!("{output}\n"))).await.is_err() {
                        // Client disconnected
                        return;
                    }
                }
            }

            if !found {
                if socket.send(Message::Text(format!("\nNo results found for {username}"))).await.is_err() {
                    // Client disconnected
                    return;
                }
            }
        }
        cmd.wait().await.unwrap();

        socket.send(Message::Close(None)).await.expect("Failed to close socket");
        println!("Finished processing request for '{username}'.");
    }
}
async fn static_lookup ( username: String ) -> Response {
    println!("Received request for {username}...");

    let proxy_link = std::env::var("PROXY_LINK").expect("Missing PROXY_LINK env variable!");
    let invalid_sites: [&str; 38] = [
        "Oracle", "8tracks", "Coders Rank", "Fiverr",
        "HackerNews", "Modelhub", "metacritic", "xHamster",
        "CNET", "YandexMusic", "HackerEarth", "OpenStreetMap", 
        "Pinkbike", "Slides", "Strava", "Archive", "CGTrader",
        "G2G", "NationStates", "IFTTT", "SoylentNews", "hunting",
        "Contently", "Euw", "OurDJTalk", "BitCoinForum", "HEXRPG",
        "Polymart", "Linktree", "GeeksforGeeks", "Kongregate", "RedTube",
        "APClips", "Heavy-R", "RocketTube", "Zhihu", "NitroType", "babyRU"
    ];

    let mut body = String::new();
    let mut cmd = Command::new("python")
        .arg("sherlock/sherlock")
        .arg("--nsfw")
        .arg("--folderoutput")
        .arg("sherlock_output")
        .arg("--timeout")
        .arg("5")
        .arg("--proxy")
        .arg(proxy_link)
        .arg("--local")
        .arg(&format!("{username}"))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Issue running the Sherlock command! Did you install with Nix?");
    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let mut stdout_lines = stdout_reader.lines();

        let mut found = false;
        while let Ok(Some(output)) = stdout_lines.next_line().await {
            if invalid_sites
                    .iter()
                    .any(|site| output.contains(site))
            {
                continue;
            }

            if output.contains("http") || output.contains("https") {
                println!("Found site for {username}: {output}");

                found = true;

                body += &format!("{output}\n");
            }
        }

        if !found {
            body += &format!("\nNo results found for {username}");
        }
    }
    cmd.wait().await.unwrap();
    
    body.into_response()
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/ws", get(handler))
        .route("/static", post(static_lookup));


    let port = std::env::var("PORT").expect("Missing PORT env variable!");
    let address = format!("0.0.0.0:{port}");

    println!("Listening on {port}, address {address}...");
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    axum::serve(listener, app).await
        .map_err(|e| anyhow!("{:?}", e))
        .context("Error in core server, terminating...")?;

    Ok(())
}
