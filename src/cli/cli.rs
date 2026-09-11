use crate::prelude::*;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub sub: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    Run {
        #[arg(short, long)]
        key: String,
    },
    Cards {
        #[command(subcommand)]
        command: CardsSub,
    },
}
