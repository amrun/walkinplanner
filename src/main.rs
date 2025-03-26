#![allow(warnings)]

use chrono::{Datelike, NaiveDate, Weekday, naive};
use serde_json::{Value, from_str};
use std::error::Error;
use std::fs;
use std::io::{BufWriter, Write, stdin, stdout};
use std::path::{Path, PathBuf};

mod company_data;
mod employee;
use crate::company_data::CompanyData;
use crate::employee::Employee;

mod file_handler; // Declares the file_builder module
use file_handler::FileHandler; // Imports FileBuilder into scope

use rand::Rng;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    let mut outputFileHandler = FileHandler::new();

    // let file_path = get_input_path();
    let file_path = "/Users/bberger/Documents/ObsidianBB/1-Privat/1-Projects/Walk-In-Planer/walkinplanner/src/input.json";

    let mut daysPlanned: i128 = 0;
    let mut daysWeekendAndHolidays: i128 = 0;

    // Read the file and parse the content
    let result =
        read_json_file(file_path).and_then(|json_content| parse_json_string(&json_content));

    match result {
        Ok(company_data) => {
            // Extract employees from company_data
            let mut employees = company_data.employees;
            // Parse holidays into Vec<NaiveDate>
            let globalHolidays: Vec<NaiveDate> = company_data
                .global_holidays
                .iter()
                .map(|s| NaiveDate::parse_from_str(s, "%d.%m.%Y"))
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|e| {
                    println!("Failed to parse holidays: {}", e);
                    Vec::new() // Fallback to empty vec
                });

            // Parse the end date string into a NaiveDate
            let mut currentDate: NaiveDate;
            let startDate_string = &company_data.from;
            match NaiveDate::parse_from_str(startDate_string, "%d.%m.%Y") {
                Ok(date) => currentDate = date,
                Err(e) => {
                    println!("Error parsing end date: {}", e);
                    return;
                }
            }

            // Parse the start date string into a NaiveDate
            let mut endDate: NaiveDate;
            let endDate_string = &company_data.to;
            match NaiveDate::parse_from_str(endDate_string, "%d.%m.%Y") {
                Ok(date) => endDate = date,
                Err(e) => {
                    println!("Error parsing start date: {}", e);
                    return;
                }
            }

            // Add Header line to csv output
            outputFileHandler.add_header_line();

            // Loop through days
            while currentDate <= endDate {
                let mut lineToAdd = String::new();
                // Get the german name of the weekday
                let (number, WeekdayName) = get_german_weekday(currentDate.weekday());
                let currentDateAndWeekdayString =
                    format_date_string(currentDate) + "," + &WeekdayName;

                // Check for weekends and continue if yes
                if (is_weekend(currentDate)) {
                    currentDate = currentDate.succ_opt().unwrap();
                    continue;
                }

                // Check for global holidays
                if (globalHolidays.contains(&currentDate)) {
                    lineToAdd.push_str(&currentDateAndWeekdayString);
                    lineToAdd.push_str(&String::from(",Ferien,Ferien"));
                    outputFileHandler.add_line(&lineToAdd);
                    currentDate = currentDate.succ_opt().unwrap();
                    continue;
                }

                lineToAdd.push_str(&currentDateAndWeekdayString);
                lineToAdd.push_str(",");

                // Plan morning employee
                employees.sort_by(|a, b| a.count.cmp(&b.count));
                let randomEmployeeNumber = rng.gen_range(0..employees.len());
                employees[randomEmployeeNumber].count = employees[randomEmployeeNumber]
                    .count
                    .checked_add(1)
                    .unwrap_or(u32::MAX);
                lineToAdd.push_str(&employees[randomEmployeeNumber].short);
                lineToAdd.push_str(",");

                // Plan afternoon employee
                employees.sort_by(|a, b| a.count.cmp(&b.count));
                let randomEmployeeNumber = rng.gen_range(0..employees.len() / 2);
                employees[randomEmployeeNumber].count = employees[randomEmployeeNumber]
                    .count
                    .checked_add(1)
                    .unwrap_or(u32::MAX);
                lineToAdd.push_str(&employees[randomEmployeeNumber].short);

                // lineToAdd.push_str(&String::from(",empty,empty"));

                outputFileHandler.add_line(&lineToAdd);

                // Check for global off days
                // if (currentDate.weekday().number_from_monday() < 4) {}

                //let (number, name) = get_german_weekday(currentDate.weekday());
                //println!("{},{}", currentDate, name);

                currentDate = currentDate.succ_opt().unwrap();
            }

            outputFileHandler.write_to_file("./output.csv");

            println!("Employees planned:");
            for e in employees {
                println!("{}: {}", e.short, e.count);
            }

            // Pause for user input
            print!("Press Enter to continue...");
            stdout().flush(); // Ensure prompt is visible
            let mut input = String::new();
            // stdin().read_line(&mut input); // Wait for Enter
        }
        Err(e) => println!("Error processing '{}': {}", file_path, e),
    }
}

fn format_date_string(date: NaiveDate) -> String {
    date.format("%d.%m.%Y").to_string()
}

fn is_weekend(date: NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

fn planEmployee(date: NaiveDate) {
    println!(
        "The weekday is: {} and is a Weekend-day: {}",
        date.weekday().num_days_from_monday(),
        is_weekend(date)
    );
    // println!("The date is: {}", date.weekday().num_days_from_monday());
    // println!("The date is: {:?}", get_german_weekday(date.weekday()));
    // let (number, name) = get_german_weekday(date.weekday());
    // println!("The date is: {}", name);
    // println!("And the next date is: {}", date.succ_opt().unwrap());
}

fn parse_json_string(json_str: &str) -> Result<CompanyData, Box<dyn std::error::Error>> {
    let company_data: CompanyData = serde_json::from_str(json_str)?;
    Ok(company_data)
}

fn read_json_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

// Function to get German weekday name and number
fn get_german_weekday(weekday: Weekday) -> (u32, &'static str) {
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

fn get_input_path() -> Result<String, Box<dyn Error>> {
    let exe_path = std::env::current_exe()?;
    let dir = exe_path
        .parent()
        .ok_or("Couldn't get executable directory")?;
    let path = dir.join("input.json");
    Ok(path.to_str().ok_or("Path is not valid UTF-8")?.to_string())
}
