#![allow(warnings)]

use chrono::{Datelike, NaiveDate, Weekday};
use serde_json::{Value, from_str};
use std::error::Error;
use std::fs;
use std::fs::File;
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

            // Now company_data and employees are available throughout this scope
            println!("Found {} employees:", employees.len());

            //TODO: put in separate function
            for employee in employees {
                println!("{:?}", employee);
            }

            // Example: Access company_data attributes
            println!(
                "Planning period: from {} to {}",
                company_data.from, company_data.to
            );
            println!("Global holidays: {:?}", company_data.global_holidays);

            // Example: Access employees later
            if let Some(first_employee) = employees.first() {
                println!("First employee's name: {}", first_employee.name);
            }
        }
        //TODO: Check how to call the writeFile function
        Err(e) => println!("Error processing '{}': {}", file_path, e),
    }
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
