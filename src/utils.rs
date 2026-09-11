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
    Ok(fs::write(path, serde_json5::to_string(content)?)?)
}
