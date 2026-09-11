use crate::prelude::*;

#[derive(Subcommand)]
pub enum CardsSub {
    Change {
        #[arg(long, short)]
        key: String,
    },
    Add {
        #[arg(long, short)]
        key: String,
        #[arg(long, short)]
        path: String,
    },
    Remove {
        #[arg(long, short)]
        key: String,
    },
    // Import,
}
