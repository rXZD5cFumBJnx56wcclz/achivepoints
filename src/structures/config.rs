use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct ConfigFile {
    pub api: Api,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct ReqSettings {
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub timeout_ms: Duration,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct ApiSettings {
    pub key: String,
    pub model: String,
    pub token: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Api {
    pub settings: ReqSettings,
    pub api: HashMap<String, ApiSettings>,
}

impl ConfigFile {
    pub fn from_json(path: PathBuf) -> RResult<Self> {
        let mut reader = fs::File::open(path)?;
        Ok(from_reader(&mut reader)?)
    }
}
