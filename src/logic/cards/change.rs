use crate::prelude::*;

pub struct LogicCardsChange<'a, 'b> {
    cards: &'b mut Cards,
    new_cards: Cards,
    clients: &'b mut Clients<'a>,
    ischanged: bool,
}

impl<'a, 'b> LogicCardsChange<'a, 'b> {
    pub fn new(k: &String, clients: &'b mut Clients<'a>, cards_map: &'b mut CardsMap) -> Self {
        let cards = cards_map.get_mut(k).unwrap();
        Self {
            new_cards: Cards::new_from(cards.info.clone()),
            cards: cards,
            clients: clients,
            ischanged: false,
        }
    }

    fn update_info(&mut self) {
        self.cards.info.version += 1;
        self.cards.info.count_elements = self.cards.cards.len();
        self.cards.info.last_pkg_version = env!("CARGO_PKG_VERSION").to_string();
    }

    pub fn interaction(&mut self) -> impl Future<Output = RResult<()>> {
        async move {
            let cli = get_cli_cycle::<CardsChangeCli>(true).await?;
            match cli.c {
                CardsChange::Remove { key } => {
                    self.cards.cards.remove(&key);
                    self.ischanged = true;
                }
                CardsChange::CreateMode => {
                    LogicCardsCreateMode::new(self.clients, &mut self.new_cards)
                        .interaction()
                        .await?;
                }
                CardsChange::Clear => {
                    self.cards.cards.clear();
                    self.cards.info.version = 0;
                }
            }
            self.ischanged = self.ischanged || !self.new_cards.cards.is_empty();
            if self.ischanged {
                println!("cards is changed");
                self.cards.cards.extend(self.new_cards.cards.clone());
                self.update_info();
            }
            println!("change cards completed\ncards info:\n{}", self.cards.info);
            self.cards.write_json()?;
            Ok(())
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::prelude_tests::prelude::*;

//     #[test]
//     fn is_changed_res_1() {
//         let mut dn = AGGR_DN();
//         let l = LogicCardsChange::new(&"rust".to_string(), &mut dn.clients, &mut dn.cards_map);
//         let info = l.cards.info.clone();
//         assert_eq_pr!(&info, &l.cards.info);
//         l.cards
//             .cards
//             .remove(&l.cards.cards.iter().next().unwrap().0.clone());
//         assert!(l.is_changed().unwrap());
//     }
// }
