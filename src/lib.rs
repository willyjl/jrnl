use chrono::{DateTime, Datelike, Local};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, enum_utils::FromStr)]
pub enum Action {
    #[enumeration(rename="open")]
    Open,
}

#[derive(Debug, PartialEq, enum_utils::FromStr)]
pub enum DateType {
    #[enumeration(rename="today")]
    Today,
}

pub struct Config {
    pub home: String,
    pub action: Action,
    pub date_type: DateType,
    pub dry_run: bool,
}

pub fn run(config: Config) {
    match config.action {
        Action::Open => {
            match config.date_type {
                DateType::Today => {
                    let (dir, filepath) = datetime_to_paths(&config.home, Local::now());
                    create_file(&dir, &filepath, config.dry_run);
                },
            }
        }
    }
}

fn datetime_to_paths(home: &String, datetime: DateTime<Local>) -> (String, String) {
    let dir = format!("{home}/{}/{:02}", datetime.year(), datetime.month());
    let filepath = format!("{dir}/{:02}.md", datetime.day());
    println!("Creating path: {filepath}");
    
    (dir, filepath)
}

fn create_file(dir: &String, filepath: &String, dry_run: bool) {
    if Path::new(filepath).exists() {
        return;
    }
    
    if dry_run {
        println!("** DRY RUN MODE **");
        println!("Create directory: {dir}");
        println!("Create filepath: {filepath}");
    } else {
        fs::create_dir_all(dir).expect("Failed to create directory");
        fs::write(filepath, "").expect("Failed to create filesystem");
        println!("Edit journal: {filepath}");
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_datetime_to_paths() {
        let home: String = ".".to_owned();
        let dt = chrono::Date::from_utc(
            chrono::NaiveDate::from_ymd(2000, 1, 1),
            chrono::offset::Utc
        ).and_hms(1, 0, 0);
        let paths = datetime_to_paths(&home, dt);
        assert_eq!(paths.0, "./2000/01");
        assert_eq!(paths.1, "./2000/01/01.md");
    }
}