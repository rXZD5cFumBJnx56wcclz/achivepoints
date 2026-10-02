use crate::prelude::*;

pub struct AggrStatic {
    pub config: ConfigFile,
    pub client: Client,
}

impl AggrStatic {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = ConfigFile::from_json("config.json".into())?;
        Ok(AggrStatic {
            client: ClientBuilder::new()
                .timeout(config.api.settings.timeout_ms)
                .build()?,
            config: config,
        })
    }
}

pub struct AggrDyn<'a> {
    pub cards_map: CardsMap,
    pub clients: Clients<'a>,
}

impl<'a> AggrDyn<'a> {
    pub fn new(aggrst: &'a AggrStatic, clients_map: &ClientsMap) -> RResult<Self> {
        Ok(Self {
            clients: Clients::new(&aggrst.client, &aggrst.config, clients_map),
            cards_map: get_cards_paths()?
                .iter()
                .map(|v| {
                    let cards = Cards::from_json(v)?;
                    Ok((cards.info.name.clone(), cards))
                })
                .collect::<RResult<_>>()?,
        })
    }
}

pub struct Aggr<'a> {
    pub st: &'a AggrStatic,
    pub dn: &'a mut AggrDyn<'a>,
}

impl<'a> Aggr<'a> {
    pub fn new(st: &'a AggrStatic, dn: &'a mut AggrDyn<'a>) -> Self {
        Self { st, dn }
    }
}
