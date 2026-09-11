#[cfg(test)]
pub mod prelude {

    pub use pretty_assertions::{assert_eq as assert_eq_pr, assert_ne as assert_ne_pr};

    pub use crate::prelude::*;

    pub static CONFIG: LazyLock<ConfigFile> =
        LazyLock::new(|| ConfigFile::from_json("config.json".into()).unwrap());
    pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
        ClientBuilder::new()
            .timeout(CONFIG.api.settings.timeout_ms)
            .build()
            .unwrap()
    });
    pub const PROMT_TEST: &str = "ownership в rust";
    pub static AGGR_ST: LazyLock<AggrStatic> = LazyLock::new(|| AggrStatic::new().unwrap());
    pub static AGGR_DN: LazyLock<fn() -> AggrDyn<'static>> =
        LazyLock::new(|| || AggrDyn::new(&AGGR_ST, &CLIENTS_MAP).unwrap());
    // pub static DNGET: LazyLock<fn() -> ()>
}
