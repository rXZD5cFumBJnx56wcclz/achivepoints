use crate::prelude::*;

pub struct LogicCardsCreateMode<'a> {
    clients: &'a Clients<'a>,
    cards: &'a mut Cards,
    count: usize,
    limit: usize,
    card: Card,
    args_state: CardsCreate,
}

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
    fn get_card(&mut self) -> impl Future<Output = RResult<Card>> {
        async move {
            Ok(match self.args_state.r#type.as_str() {
                "manualy" => Card::new(
                    self.args_state.key.clone(),
                    self.args_state.ask.clone(),
                    self.args_state.answer.clone(),
                ),
                k => {
                    let v = &self.clients.0[k];
                    wrap_err(|| {
                        v.client.req(
                            &v.conn,
                            Promts::new(
                                PROMTS[self.args_state.type_promt_system.as_str()],
                                RT[self.args_state.response_type.as_str()],
                                &self.args_state.ask,
                            ),
                        )
                    })
                    .await
                    .content
                }
            })
        }
    }

    fn args_impl(&mut self) {
        if self.args_state.qty_sep {
            self.limit += self.args_state.qty;
        } else {
            self.limit = self.args_state.qty;
        }
    }

    pub fn new(clients: &'a Clients, cards: &'a mut Cards) -> Self {
        Self {
            cards,
            clients,
            count: Default::default(),
            limit: Default::default(),
            card: Default::default(),
            args_state: Default::default(),
        }
    }

    pub fn add(&mut self) -> impl Future<Output = RResult<()>> {
        async move {
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
                self.args_impl();
                self.card = self.get_card().await?;
                if !self.args_state.uncheck {
                    println!("{}", self);
                    let cli = get_cli_cycle::<CardsCreateModeCli>(true).await?;
                    match cli.c {
                        CardsCreateMode::Add => self.add().await?,
                        CardsCreateMode::Create(a) => {
                            self.args_state = a;
                        }
                        CardsCreateMode::Delete => self.delete().await?,
                        CardsCreateMode::Repeat => {}
                    }
                } else {
                    self.add().await?;
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
        l.get_card().await.unwrap();
    }

    #[test]
    fn args_impl_res_1() {
        let mut dn = AGGR_DN();
        let mut l = LogicCardsCreateMode::new(&dn.clients, dn.cards_map.get_mut("rust").unwrap());
        assert_eq_pr!(l.limit, 0);
        assert_eq_pr!(l.args_state.qty, 1);
        l.args_impl();
        assert_eq_pr!(l.limit, 1);
    }
}
