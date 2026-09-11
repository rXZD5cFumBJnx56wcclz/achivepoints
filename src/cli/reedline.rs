use reedline::{DefaultPrompt, FileBackedHistory, Reedline, Signal};

use crate::prelude::*;

pub async fn get_cli_cycle<T: Parser>(fictive: bool) -> RResult<T> {
    let mut line_editor = Reedline::create().with_history(Box::new(FileBackedHistory::with_file(
        1000,
        "history_command.txt".into(),
    )?));
    loop {
        match line_editor.read_line(&DefaultPrompt::default()) {
            Ok(Signal::Success(input)) => {
                let res = T::try_parse_from({
                    let mut v = shell_words::split(&input.trim())?;
                    if fictive {
                        v.insert(0, "fictive".to_string());
                    }
                    v
                });
                if res.is_ok() {
                    return Ok(res?);
                } else {
                    eprintln!("{}", res.err().unwrap())
                }
            }
            Ok(Signal::CtrlD) => continue,
            Ok(Signal::CtrlC) => panic!("ctrlC => exit"),
            Err(e) => eprintln!("{e}"),
        };
    }
}
