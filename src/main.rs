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

fn main() {
    let mut outputContent = String::new();
    let file_path = "/Users/bberger/Documents/ObsidianBB/1-Privat/1-Projects/Walk-In-Planer/walkinplanner/src/input.json";

    // Read and parse the data
    let result =
        read_json_file(file_path).and_then(|json_content| parse_json_string(&json_content));

    match result {
        Ok(company_data) => {
            // Extract employees from company_data
            let employees = &company_data.employees;

            // Now company_data and employees are available throughout this scope
            outputContent.push_str(&format!("Found {} employees:", employees.len()));
            for employee in employees {
                outputContent.push_str(&format!("{:?}", employee));
            }

            // Example: Access company_data attributes
            outputContent.push_str(&format!(
                "Planning period: from {} to {}",
                company_data.from, company_data.to
            ));
            outputContent.push_str(&format!(
                "Global holidays: {:?}",
                company_data.global_holidays
            ));

            // Example: Access employees later
            if let Some(first_employee) = employees.first() {
                outputContent.push_str(&format!("First employee's name: {}", first_employee.name));
            }
        }
        //TODO: Check how to call the writeFile function
        Err(e) => println!("Error processing '{}': {}", file_path, e),
    }

    /*match read_json_file(file_path) {
        Ok(json_content) => {
            match parse_json_string(&json_content) {
                Ok(employees) => {
                    println!("Found {} employees:", employees.len());
                    for employee in &employees {
                        println!("{:?}", employee);
                        //println!("{}", employee.name + employee.surname);
                    }
                }
                Err(e) => println!("Error parsing JSON: {}", e),
            }
        }
        Err(e) => println!("Error reading file '{}': {}", file_path, e),
    } */
}

fn writeFile(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = content;
    let file_path = "output.txt";

    // Create or open the file (overwrites by default)
    let mut file = File::create(file_path)?;

    // Write the string content
    file.write_all(content.as_bytes())?;

    println!("Successfully wrote to {}", file_path);
    Ok(())
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
