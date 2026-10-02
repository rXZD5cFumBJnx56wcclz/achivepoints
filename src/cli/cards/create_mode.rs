use crate::prelude::*;
use struct_patch::Patch;

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
    Create(CardsCreatePatch),
}

#[derive(Args, Clone, Debug, Patch, PartialEq)]
#[patch(attribute(derive(Debug, Default, Args, Clone)))]
pub struct CardsCreate {
    #[patch(attribute(arg(short, long,)))]
    pub key: String,
    #[patch(attribute(arg(short = 's', long,)))]
    pub ask: String,
    #[patch(attribute(arg(short = 'a', long,)))]
    pub answer: String,
    #[patch(attribute(arg(short = 'e', long,)))]
    pub explanation: String,
    // type: manualy
    #[patch(attribute(arg(short, long,)))]
    pub r#type: String,
    #[patch(attribute(arg(short, long)))]
    pub dont_save: bool,
    #[patch(attribute(arg(short, long,)))]
    pub qty: usize,
    #[patch(attribute(arg(long)))]
    pub qty_add: bool,
    #[patch(attribute(arg(long,)))]
    pub type_promt_system: String,
    // variant, text,
    #[patch(attribute(arg(long,)))]
    pub response_type: String,
}

impl Default for CardsCreate {
    fn default() -> Self {
        Self {
            ask: "Ask".to_string(),
            answer: "Answer".to_string(),
            qty: 1,
            qty_add: false,
            r#type: "manualy".to_string(),
            key: "Theme".to_string(),
            dont_save: false,
            type_promt_system: "all".to_string(),
            response_type: "variant".to_string(),
            explanation: "Explanation".to_string(),
        }
    }
}
