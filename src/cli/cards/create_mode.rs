use crate::prelude::*;

#[derive(Parser)]
pub struct CardsCreateModeCli {
    #[command(subcommand)]
    pub c: CardsCreateMode,
}

#[derive(Subcommand)]
pub enum CardsCreateMode {
    // addition
    #[command(alias = "a")]
    Add,
    // delete answer
    #[command(alias = "d")]
    Delete,
    // repeat
    #[command(alias = "r")]
    Repeat,
    Create(CardsCreate),
}

#[derive(Args, Clone)]
pub struct CardsCreate {
    #[arg(short = 's', long, default_value = "")]
    pub ask: String,
    #[arg(short = 'a', long, default_value = "")]
    pub answer: String,
    #[arg(short, long, default_value = "1")]
    pub qty: usize,
    #[arg(long)]
    pub qty_sep: bool,
    // type: manualy
    #[arg(short, long, default_value = "manualy")]
    pub r#type: String,
    #[arg(short, long)]
    pub uncheck: bool,
    #[arg(short, long, default_value = "")]
    pub key: String,
    #[arg(long, default_value = "all")]
    pub type_promt_system: String,
    // variant, text,
    #[arg(long, default_value = "variant")]
    pub response_type: String,
}

impl Default for CardsCreate {
    fn default() -> Self {
        Self {
            ask: "Ask".to_string(),
            answer: "Answer".to_string(),
            qty: 1,
            qty_sep: false,
            r#type: "manualy".to_string(),
            uncheck: false,
            key: "Theme".to_string(),
            type_promt_system: "all".to_string(),
            response_type: "variant".to_string(),
        }
    }
}
