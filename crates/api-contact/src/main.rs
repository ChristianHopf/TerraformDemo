mod args;
mod endpoints;

use args::{Cli, Command };
use clap::Parser;

#[tokio::main]
async fn main(){
    server::logging::start("lettre=INFO,DEBUG");
    match Cli::parse().command{
        Command::Server { email, listen } => endpoints::run(&listen, email).await.unwrap(),
    }
}
