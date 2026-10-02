use std::sync::{Arc, RwLock};

use struct_patch::Patch;
use structdiff::{Difference, StructDiff};

use crate::prelude::*;

pub struct LogicCardsCreateMode<'a> {
    clients: &'a Clients<'a>,
    cards: &'a mut Cards,
    count: usize,
    limit: usize,
    card: Card,
    // card_use: Card,
    args: CardsCreate,
    args_use: CardsCreate,
    args_patch: CardsCreatePatch,
}

// input args_patch
// args_use = args_patch
// args_use -> card_use
//

impl Display for LogicCardsCreateMode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "card:\n\n{}\n\nlimit: {}\ncount: {}",
            self.card, self.limit, self.count
        )
    }
}

impl<'a> LogicCardsCreateMode<'a> {
    fn card_patch_manualy(&self) -> CardPatch {
        CardPatch {
            key: if self.args.key != self.args_use.key {
                Some(self.args_use.key.clone())
            } else {
                None
            },
            ask: if self.args.ask != self.args_use.ask {
                Some(self.args_use.ask.clone())
            } else {
                None
            },
            answer: if self.args.answer != self.args_use.answer {
                Some(self.args_use.answer.clone())
            } else {
                None
            },
            explanation: if self.args.explanation != self.args_use.explanation {
                Some(self.args_use.explanation.clone())
            } else {
                None
            },
        }
    }

    fn get_card_patch(&mut self) -> impl Future<Output = RResult<CardPatch>> {
        async move {
            Ok(match self.args_use.r#type.as_str() {
                "manualy" => self.card_patch_manualy(),
                k => {
                    let v = &self.clients.0[k];
                    v.client
                        .req(
                            &v.conn,
                            Promts::new(
                                PROMTS[self.args_use.type_promt_system.as_str()],
                                RT[self.args_use.response_type.as_str()],
                                &self.args_use.ask,
                            ),
                        )
                        .await?
                        .content
                }
            })
        }
    }

    fn cycle_runner(&mut self) {
        if self.args.qty_add {
            self.limit += self.args.qty;
        } else {
            self.limit = self.args.qty;
        }
    }

    fn args_assign(&mut self) {
        self.args_use = self.args.clone();
        self.args_use.apply(self.args_patch.clone());
        self.args_patch = Default::default();
        if self.args_use.dont_save {
            return;
        }
        self.args = self.args_use.clone();
    }

    fn card_assign(&mut self, patch: CardPatch) {
        self.card.apply(patch);
    }

    pub fn new(clients: &'a Clients, cards: &'a mut Cards) -> Self {
        Self {
            cards,
            clients,
            count: Default::default(),
            limit: Default::default(),
            card: Default::default(),
            // card_use: Default::default(),
            args: Default::default(),
            args_use: Default::default(),
            // card_patch: Default::default(),
            args_patch: Default::default(),
        }
    }

    pub fn add(&mut self) -> impl Future<Output = RResult<()>> {
        async move {
            // self.card = self.card_use.clone();
            self.cards
                .cards
                .insert(self.card.clone().key, self.card.clone());
            self.count += 1;
            Ok(())
        }
    }

    pub fn delete(&mut self) -> impl Future<Output = RResult<()>> {
        async move {
            self.count += 1;
            Ok(())
        }
    }

    pub fn interaction(&mut self) -> impl Future<Output = RResult<()>> {
        async move {
            loop {
                self.args_assign();
                let card_patch = self.get_card_patch().await?;
                self.card_assign(card_patch);
                self.cycle_runner();
                println!("{}", self);
                let cli = get_cli_cycle::<CardsCreateModeCli>(true).await?;
                match cli.c {
                    CardsCreateMode::Add => self.add().await?,
                    CardsCreateMode::Create(a) => {
                        self.args_patch = a.clone();
                    }
                    CardsCreateMode::Delete => self.delete().await?,
                    CardsCreateMode::Repeat => {
                        continue;
                    }
                }
                if self.count >= self.limit {
                    println!("create mode completed");
                    break;
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[tokio::test]
    async fn get_card_res_1() {
        let mut dn = AGGR_DN();
        let mut l = LogicCardsCreateMode::new(&dn.clients, dn.cards_map.get_mut("rust").unwrap());
        l.get_card_patch().await.unwrap();
    }

    #[test]
    fn args_impl_res_1() {
        let mut dn = AGGR_DN();
        let mut l = LogicCardsCreateMode::new(&dn.clients, dn.cards_map.get_mut("rust").unwrap());
        assert_eq_pr!(l.limit, 0);
        assert_eq_pr!(l.args.qty, 1);

        l.cycle_runner();
        assert_eq_pr!(l.limit, 1);
    }
}
