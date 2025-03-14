#![allow(warnings)]

use chrono::{Datelike, NaiveDate, Weekday, naive};
use serde_json::{Value, from_str};
use std::error::Error;
use std::fs;
use std::io::{BufWriter, Write, stdout};
use std::path::Path;

mod company_data;
mod employee;
use crate::company_data::CompanyData;
use crate::employee::Employee;

mod file_handler; // Declares the file_builder module
use file_handler::FileHandler; // Imports FileBuilder into scope

fn main() {
    let mut outputFileHandler = FileHandler::new();
    let file_path = "/Users/bberger/Documents/ObsidianBB/1-Privat/1-Projects/Walk-In-Planer/walkinplanner/src/input.json";

    // Read the file and parse the content
    let result =
        read_json_file(file_path).and_then(|json_content| parse_json_string(&json_content));

    match result {
        Ok(company_data) => {
            // Extract employees from company_data
            let mut employees = &company_data.employees;
            let mut currentDate: NaiveDate;
            let mut endDate: NaiveDate;

            let endDate_string = &company_data.to;

            // Parse the string into a NaiveDate
            match NaiveDate::parse_from_str(endDate_string, "%d.%m.%Y") {
                Ok(date) => endDate = date,
                Err(e) => {
                    println!("Error parsing date: {}", e);
                    return;
                }
            }

            let startDate_string = &company_data.from;

            // Parse the string into a NaiveDate
            match NaiveDate::parse_from_str(startDate_string, "%d.%m.%Y") {
                Ok(date) => currentDate = date,
                Err(e) => {
                    println!("Error parsing date: {}", e);
                    return;
                }
            }

            // Loop through days
            while currentDate <= endDate {
                println!("{}", currentDate);
                currentDate = currentDate.succ_opt().unwrap();
            }

            // TODO: for example purposes only. Remove when ready
            for employee in employees {
                println!("{}", employee.count);
                println!("{}", employee.short);
                println!("{}", employee.name);
            }

            // Example: Access company_data attributes
            println!(
                "Planning period: from {} to {}",
                company_data.from, company_data.to
            );
            println!("Global holidays: {:?}", company_data.global_holidays);

            // outputFileHandler.add_header_line();
            // outputFileHandler.write_to_file("./output.csv");
        }
        Err(e) => println!("Error processing '{}': {}", file_path, e),
    }
}

fn is_weekend(date: NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

fn planDate(date: NaiveDate) {
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
