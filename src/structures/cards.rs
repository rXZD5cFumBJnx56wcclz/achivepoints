use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Card {
    pub key: String,
    pub ask: String,
    pub answer: String,
}

impl Card {
    pub fn new(key: String, ask: String, answer: String) -> Self {
        Self { key, ask, answer }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self {
            key: "Theme".to_string(),
            ask: "Ask".to_string(),
            answer: "Answer".to_string(),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "key: {}\n\nask: {}\n\nanswer: {}",
            &self.key, &self.ask, &self.answer
        )?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InfoData {
    pub version: usize,
    pub name: String,
    pub path: PathBuf,
    pub count_elements: usize,
}

impl InfoData {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            version: 0,
            name,
            path,
            count_elements: 0,
        }
    }
}

impl Display for InfoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "version: {}\ncount elements: {}\n name: {}\npath: {}",
            self.version,
            self.count_elements,
            self.name,
            self.path.clone().into_string().unwrap()
        )?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cards {
    pub info: InfoData,
    pub cards: HashMap<String, Card>,
}

impl Cards {
    pub fn from_json(path: &PathBuf) -> RResult<Self> {
        let f = fs::read_to_string(path)?;
        Ok(serde_json5::from_str(&f)?)
    }

    pub fn write_json(&self) -> RResult<()> {
        write_json(&self.info.path, self)
    }

    pub fn to_vec(&self) -> Vec<(&String, &Card)> {
        self.cards.iter().collect::<Vec<_>>()
    }
}

pub type CardsAsVec<'a> = Vec<(&'a String, &'a Card)>;
pub type CardsMap = HashMap<String, Cards>;
