use std::env::{var, args};
use jrnl::{Action, DateType, Config, run};

fn main() {
    let journal_path = var("JOURNAL_PATH").expect("JOURNAL_PATH is not yet set");

    let args: Vec<String> = args().collect();
    let action: Action = args[1].parse().expect("Action is unknown");
    let date_type: DateType = args[2].parse().expect("Date type is unknown");

    run(Config {
        home: journal_path,
        action: action,
        date_type: date_type,
        dry_run: true
    });
}