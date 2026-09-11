use crate::prelude::*;

pub struct LogicCardsChange<'a, 'b> {
    cards: &'b mut Cards,
    clients: &'b mut Clients<'a>,
}

impl<'a, 'b> LogicCardsChange<'a, 'b> {
    pub fn new(k: &String, clients: &'b mut Clients<'a>, cards_map: &'b mut CardsMap) -> Self {
        Self {
            cards: cards_map.get_mut(k).unwrap(),
            clients: clients,
        }
    }

    fn update_info(&mut self) -> RResult<()> {
        if Cards::from_json(&self.cards.info.path)?
            .cards
            .iter()
            .any(|(k, v)| self.cards.cards.get(k).is_none() || !self.cards.cards[k.as_str()].eq(v))
        {
            self.cards.info.version += 1;
            self.cards.info.count_elements = self.cards.cards.len();
        }
        Ok(())
    }

    pub fn interaction(&mut self) -> impl Future<Output = RResult<()>> {
        async move {
            let cli = get_cli_cycle::<CardsChangeCli>(true).await?;
            match cli.c {
                CardsChange::Remove { key } => {
                    self.cards.cards.remove(&key);
                }
                CardsChange::CreateMode => {
                    LogicCardsCreateMode::new(self.clients, self.cards)
                        .interaction()
                        .await?;
                }
                CardsChange::Clear => {
                    self.cards.cards.clear();
                    self.cards.info.version = 0;
                }
            }
            self.update_info()?;
            println!("change cards completed\ncards info:\n{}", self.cards.info);
            self.cards.write_json()?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[test]
    fn update_info_res_1() {
        let mut dn = AGGR_DN();
        let mut l = LogicCardsChange::new(&"rust".to_string(), &mut dn.clients, &mut dn.cards_map);
        let info = l.cards.info.clone();
        l.update_info().unwrap();
        assert_eq_pr!(&info, &l.cards.info);
        l.cards
            .cards
            .remove(&l.cards.cards.iter().next().unwrap().0.clone());
        l.update_info().unwrap();
        assert_ne_pr!(&info, &l.cards.info);
    }
}
