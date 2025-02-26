use chrono::{NaiveDate, Weekday, Datelike};
use std::fs;
use serde_json::{Value, from_str};
use std::error::Error;


use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};


fn main() {
match read_json_file("/Users/bberger/Documents/ObsidianBB/1-Privat/1-Projects/Walk-In-Planer/walkinplanner/src/input.json") {
        Ok(json) => println!("JSON contents: {:?}", json),
        Err(e) => eprintln!("Error reading JSON file: {}", e),
    }
/*
    let dates = vec!["15.03.2023", "22.07.2024", "01.12.2022", "09.05.2025"];
    process_dates(&dates);


    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
    */

}

// Function to get German weekday name and number
fn get_german_weekday_info(weekday: Weekday) -> (u32, &'static str) {
    let number = weekday.number_from_monday(); // Monday = 1, Sunday = 7
    let name = match weekday {
        Weekday::Mon => "Montag",
        Weekday::Tue => "Dienstag",
        Weekday::Wed => "Mittwoch",
        Weekday::Thu => "Donnerstag",
        Weekday::Fri => "Freitag",
        Weekday::Sat => "Samstag",
        Weekday::Sun => "Sonntag",
    };
    (number, name)


}

// Function to process and print dates
fn process_dates(dates: &[&str]) {
    for date_str in dates {
        match NaiveDate::parse_from_str(date_str, "%d.%m.%Y") {
            Ok(date) => {
                let (number, name) = get_german_weekday_info(date.weekday());
                println!("{} ist ein {} (Wochentag-Nummer: {})", date_str, name, number);
            }
            Err(e) => {
                println!("Fehler beim Parsen von {}: {}", date_str, e);
            }
        }
    }
}


fn read_json_file(file_path: &str) -> Result<Value, Box<dyn Error>> {
    // Read the file contents into a string
    let contents = fs::read_to_string(file_path)?;
    
    // Parse the string into a JSON Value
    let json: Value = from_str(&contents)?;
    
    Ok(json)
}