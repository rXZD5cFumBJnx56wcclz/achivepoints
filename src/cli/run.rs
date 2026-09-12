use crate::prelude::*;

#[derive(Parser, Debug)]
pub struct RunCli {
    pub answer: Option<String>,
    #[command(subcommand)]
    pub c: Option<RunSub>,
}

// #[arg(long, default_value = "eq")]
//     pub comparison_answers: String,

#[derive(Subcommand, Debug)]
pub enum RunSub {
    #[command(alias = "e")]
    Exit,
}
