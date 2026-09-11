use crate::prelude::*;

#[derive(Parser)]
pub struct RunCli {
    pub answer: Option<String>,
    #[command(subcommand)]
    pub c: Option<RunSub>,
}

// #[arg(long, default_value = "eq")]
//     pub comparison_answers: String,

#[derive(Subcommand)]
pub enum RunSub {
    #[command(alias = "e")]
    Exit,
}
