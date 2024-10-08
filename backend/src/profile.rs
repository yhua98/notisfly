use clap::Parser;

use crate::cli;

#[derive(Debug, Clone)]
pub enum Mode {
    DEV,
    #[allow(unused)]
    PROD,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub db_url: String,
    pub mode: Mode,
    pub port: String,
}

impl Profile {
    pub fn parse() -> Result<Self, String> {
        // parse command line arguments
        let args = cli::Cli::parse();
        // loaded .env file by dotenvy
        dotenvy::dotenv().ok();

        // check if db_url is provided in command line arguments
        // override DATABASE_URL with db_url if .env file is set and db_url is provided
        if let Some(db_url) = args.db_url {
            std::env::set_var("DATABASE_URL:", db_url);
        }
        let db_url = dotenvy::var("DATABASE_URL");
        if db_url.is_err() {
            return Err("DATABASE_URL is not set".to_string());
        }
        let db_url = db_url.unwrap();

        // check if port is provided in command line arguments
        // override PORT with port if .env file is set and port is provided
        let port = if let Some(port) = args.port {
            port
        } else {
            let port = dotenvy::var("PORT");
            if port.is_err() {
                "6543".to_string()
            } else {
                port.unwrap()
            }
        };

        Ok(Self {
            db_url,
            mode: Mode::DEV,
            port,
        })
    }
}

impl Profile {
    pub fn banner(self: Self) -> String {
        format!("=========================================================================================
 ________   ________  _________  ________ ___           ___    ___ 
|\\   ___  \\|\\   __  \\|\\___   ___\\\\  _____\\\\  \\         |\\  \\  /  /|
\\ \\  \\\\ \\  \\ \\  \\|\\  \\|___ \\  \\_\\ \\  \\__/\\ \\  \\        \\ \\  \\/  / /     version: 0.1.0
 \\ \\  \\\\ \\  \\ \\  \\\\\\  \\   \\ \\  \\ \\ \\   __\\\\ \\  \\        \\ \\    / /      port: {port}
  \\ \\  \\\\ \\  \\ \\  \\\\\\  \\   \\ \\  \\ \\ \\  \\_| \\ \\  \\____    \\/  /  /       mode: {mode}
   \\ \\__\\\\ \\__\\ \\_______\\   \\ \\__\\ \\ \\__\\   \\ \\_______\\__/  / /    
    \\|__| \\|__|\\|_______|    \\|__|  \\|__|    \\|_______|\\___/ /     
                                                      \\|___|/      
=========================================================================================", port=self.port, mode=match self.mode {
            Mode::DEV => "DEV",
            Mode::PROD => "PROD",
        })
    }
}
