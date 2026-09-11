use crate::prelude::*;

pub struct LogicCards<'a> {
    cards_map: &'a mut CardsMap,
    clients: &'a mut Clients<'a>,
    command: &'a CardsSub,
}

impl<'a> LogicCards<'a> {
    pub fn new(aggr: &'a mut Aggr<'a>, command: &'a CardsSub) -> Self {
        Self {
            cards_map: &mut aggr.dn.cards_map,
            clients: &mut aggr.dn.clients,
            command,
        }
    }

    pub fn add(&mut self, k: String, path: &PathBuf) -> impl Future<Output = RResult<()>> {
        async move {
            self.cards_map.insert(k, Cards::from_json(path)?);
            Ok(())
        }
    }

    pub fn interaction(&mut self) -> impl Future<Output = RResult<()>> {
        async move {
            match self.command {
                CardsSub::Add { key, path } => {
                    self.add(key.clone(), &path.into()).await?;
                }
                CardsSub::Change { key } => {
                    LogicCardsChange::new(key, self.clients, self.cards_map)
                        .interaction()
                        .await?;
                }
                CardsSub::Remove { key } => {
                    self.cards_map.remove(key);
                }
            }
            Ok(())
        }
    }
}
