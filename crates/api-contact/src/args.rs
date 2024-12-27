use clap::{Parser, Subcommand};
use proto::EmailConfig;

#[derive(Subcommand, Debug)]
pub enum Command {
    Server {
        // Email notif config
        #[clap(flatten)]
        email: EmailConfig,

        // Net listening address of HTTP server
        #[clap(short, long, default_value = "0.0.0.0:8000", env = "LISTEN")]
        listen: String,
    }
}

#[derive(Parser, Debug)]
#[clap(name="api-contact", about = "API of the contact form")]
pub struct Cli {
    // Command to execute
    #[clap(subcommand)]
    pub command: Command,
    // Logging level
    #[clap(short, long, default_value = "", env = "RUST_LOG")]
    pub log_level: String,
}
