use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

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
            "points: {}\ntime ms: {}\ntime sec: {}\ntime min: {:.4}\ninfo_data: {}",
            self.points,
            self.time_ms.as_millis(),
            self.time_ms.as_secs(),
            self.time_ms.as_secs_f64() / 60.,
            self.info_data
        )?;
        Ok(())
    }
}

impl SessionStat {
    pub fn push_file(&self) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("stat.jsonl")?;
        serde_json::to_writer(&mut file, self)?;
        file.write_all(b"\n")?;
        file.flush()?;
        Ok(())
    }

    pub fn from_file() -> RResult<Vec<SessionStat>> {
        let file = File::open("stat.jsonl")?;
        let reader = BufReader::new(file);

        let mut stat = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue; // пропускаем пустые строки
            }

            let card: Self = serde_json::from_str(&line)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

            stat.push(card);
        }

        Ok(stat)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_file_res_1() {
        let _ = SessionStat::from_file().unwrap();
    }
}
