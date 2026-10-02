use crate::prelude::*;

pub async fn wrap_err<T>(func: impl AsyncFn() -> RResult<T>) -> T {
    loop {
        let res = func().await;
        if let Ok(value) = res {
            return value;
        } else {
            println!("{}", res.err().unwrap());
        }
    }
}

pub fn write_json<T: Serialize>(path: &PathBuf, content: &T) -> RResult<()> {
    Ok(fs::write(path, serde_json::to_string_pretty(content)?)?)
}

pub fn get_cards_paths() -> RResult<Vec<PathBuf>> {
    Ok(fs::read_dir("cards")?.map(|v| {
        let path = v?.path();
        if path.is_file() {
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }).collect::<RResult<Vec<Option<PathBuf>>>>()?.into_iter().filter_map(|v| v).collect())
}
