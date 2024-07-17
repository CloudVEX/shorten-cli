use std::process;

use clap::{Parser, Subcommand};
use reqwest::{Client, Error, Response};
use tokio;
use toml::Value;

#[derive(Parser)]
#[command(
    name = "shorten-cli",
    version = "1.0",
    author = "Cloud",
    about = "A CLI for my url shortening api"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    RM { url: String },
    CR { url: String },
    GET { url: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let raw_config_path = match dirs::config_dir() {
        Some(path) => path,
        None => {
            println!("Your file structure or operating system isn't supported.");
            process::exit(1);
        }
    };
    let config_path = format!("{:?}/shorten-cli/cli.toml", raw_config_path).replace("\"", "");
    let config_str = match std::fs::read_to_string(config_path) {
        Ok(str) => str,
        Err(_) => {
            println!("Failed to read config file, copy the example config to your config path if you didn't already.");
            process::exit(1);
        }
    };
    let value = match config_str.parse::<Value>() {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to parse the config, possibly because of syntax errors.");
            process::exit(1);
        }
    };

    let client = Client::new();
    let shorten_config = value["shorten"].clone();
    let server = shorten_config["server"].as_str();
    let server = match server {
        Some(value) => value,
        None => "l.cloudvex.de",
    };

    match cli.command {
        Commands::RM { url } => {
            let username = match shorten_config["username"].as_str() {
                Some(value) => value,
                None => {
                    println!("Username not set in the config.");
                    process::exit(1);
                }
            };
            let password = match shorten_config["password"].as_str() {
                Some(value) => value,
                None => {
                    println!("Password not set in the config.");
                    process::exit(1);
                }
            };

            let body = format!(
                "{{\"username\": \"{}\", \"password\": \"{}\"}}",
                username, password
            );
            let response = decode_response(
                client
                    .delete(format!("https://{}/{}", server, url))
                    .body(body)
                    .send()
                    .await,
            )
            .await;

            println!("{}", response)
        }
        Commands::CR { url } => {
            let body = format!("{{\"url\": \"{}\"}}", url);
            let response = decode_response(
                client
                    .post(format!("https://{}/shorten", server))
                    .body(body)
                    .send()
                    .await,
            )
            .await;

            println!("\nNo server set, using fallback, please consider hosting your own instance of https://github.com/CloudVEX/url-short\n");
            println!("https://{}/{}", server, response);
        }
        Commands::GET { url } => {
            let response = match client
                .get(format!("https://{}/{}", server, url))
                .send()
                .await
            {
                Ok(value) => value,
                Err(_) => {
                    println!("Unable to connect to the server.");
                    process::exit(1);
                }
            };

            println!("{}", response.url())
        }
    }

    Ok(())
}

async fn decode_response(raw: Result<Response, Error>) -> String {
    let raw = match raw {
        Ok(value) => value,
        Err(_) => {
            println!("Unable to connect to the server.");
            process::exit(1);
        }
    };

    return match raw.text().await {
        Ok(value) => value,
        Err(_) => {
            println!("Unable to decode the response from the server.");
            process::exit(1);
        }
    };
}
