use clap::{Parser, Subcommand};
use reqwest::Client;
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
    let config_path =
        format!("{:?}/cli.toml", dirs::home_dir().expect("Unsupported OS")).replace("\"", "");
    let config_str = std::fs::read_to_string(config_path).expect("Failed to read config file");
    let value = config_str.parse::<Value>().expect("Failed to parse TOML");

    let client = Client::new();
    let shorten_config = value["shorten"].clone();

    match cli.command {
        Commands::RM { url } => {
            let username = shorten_config["username"]
                .as_str()
                .expect("Username not set");
            let password = shorten_config["password"]
                .as_str()
                .expect("Password not set");

            let body = format!(
                "{{\"username\": \"{}\", \"password\": \"{}\"}}",
                username, password
            );
            let response = client
                .delete(format!("https://l.cloudvex.de/{}", url))
                .body(body)
                .send()
                .await?
                .text()
                .await?;

            println!("{}", response)
        }
        Commands::CR { url } => {
            let body = format!("{{\"url\": \"{}\"}}", url);
            let response = client
                .post("https://l.cloudvex.de/shorten")
                .body(body)
                .send()
                .await?
                .text()
                .await?;

            println!("https://l.cloudvex.de/{}", response);
        }
        Commands::GET { url } => {
            let response = client
                .get(format!("https://l.cloudvex.de/{}", url))
                .send()
                .await?;

            println!("{}", response.url())
        }
    }

    Ok(())
}
