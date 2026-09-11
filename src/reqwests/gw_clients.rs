use crate::prelude::*;

pub type ClientsMap = HashMap<&'static str, fn() -> Box<dyn ClientCardResp>>;

pub static CLIENTS_MAP: LazyLock<ClientsMap> = LazyLock::new(|| {
    HashMap::from_iter([(
        "groq",
        (|| Box::new(GroqClient)) as fn() -> Box<dyn ClientCardResp>,
    )])
});
pub struct ClientWrap<'a> {
    pub client: Box<dyn ClientCardResp>,
    pub conn: ApiConnector<'a>,
}
pub struct Clients<'a>(pub HashMap<&'a str, ClientWrap<'a>>);

impl<'a> Clients<'a> {
    pub fn new(cl: &'a Client, config: &'a ConfigFile, clients_map: &ClientsMap) -> Self {
        Self(
            config
                .api
                .api
                .iter()
                .map(|(k, v)| {
                    (
                        k.as_str(),
                        ClientWrap {
                            conn: ApiConnector::new(cl, v),
                            client: clients_map[v.key.as_str()](),
                        },
                    )
                })
                .collect(),
        )
    }
}
