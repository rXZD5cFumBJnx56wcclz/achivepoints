use crate::prelude::*;

#[derive(Parser)]
pub struct CardsChangeCli {
    #[command(subcommand)]
    pub c: CardsChange,
}

#[derive(Subcommand)]
pub enum CardsChange {
    CreateMode,
    Remove {
        #[arg(short, long)]
        key: String,
    },
    Clear,
}
