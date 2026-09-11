use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionStat {
    pub points: i64,
    pub time_ms: Duration,
    pub info_data: InfoData,
}

impl Display for SessionStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "points: {}\ntime ms: {}\n, info_data: {}",
            self.points,
            self.time_ms.as_millis(),
            self.info_data
        )?;
        Ok(())
    }
}

impl SessionStat {
    pub fn push_file(&self) -> Result<(), Box<dyn Error>> {
        fs::write("stat.jsonl", to_string(self)?)?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct SessionRuntime {
    pub points: i64,
    pub time: Instant,
}

impl SessionRuntime {
    pub fn new() -> Self {
        Self {
            points: 0,
            time: Instant::now(),
        }
    }
    pub fn to_stat(self, info_data: InfoData) -> SessionStat {
        SessionStat {
            points: self.points,
            time_ms: self.time.elapsed(),
            info_data: info_data,
        }
    }
}
