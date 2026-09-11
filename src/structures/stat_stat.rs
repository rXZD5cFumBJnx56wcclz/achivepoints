use crate::prelude::*;

pub struct StatStat {
    pub max_points: i64,
    pub min_time_max_points: u128,
}

impl Display for StatStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "max points: {}\nmin_time_max_points: {}\n",
            self.max_points, self.min_time_max_points
        )
    }
}

impl StatStat {
    pub fn from_stat(stat: &Vec<SessionStat>) -> Self {
        Self {
            max_points: stat.iter().max_by_key(|v| v.points).unwrap().points,
            min_time_max_points: stat
                .iter()
                .min_by_key(|v| v.time_ms)
                .unwrap()
                .time_ms
                .as_millis(),
        }
    }
}
