use clap::Parser;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Database url
    #[arg(short, long)]
    pub db_url: Option<String>,
    /// Port number
    #[arg(short, long)]
    pub port: Option<String>,
}