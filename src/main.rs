#![allow(warnings)]

use chrono::{Datelike, NaiveDate, Weekday, naive};
use serde_json::{Value, from_str};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{BufWriter, Write, stdin, stdout};
use std::path::{Path, PathBuf};

mod company_data;
mod employee;
use crate::company_data::CompanyData;
use crate::employee::Employee;

mod file_handler;
use file_handler::FileHandler;

use rand::thread_rng;
use rand::{Rng, random};

fn main() {
    let mut outputFileHandler = FileHandler::new();

    // let file_path = get_input_path();
    let file_path = "/Users/bberger/Code/walkinplanner/src/input.json";

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
                    lineToAdd.push_str(&String::from(",Feiertag,"));
                    outputFileHandler.add_line(&lineToAdd);
                    currentDate = currentDate.succ_opt().unwrap();
                    continue;
                }

                // Add current date and weekday
                lineToAdd.push_str(&currentDateAndWeekdayString);
                lineToAdd.push_str(",");

                // Plan morning employee
                let mut key = format!("{}v", currentDate.weekday().number_from_monday());
                if company_data.fix_days[0].get(&key).unwrap_or(&0) > &0 {
                    let id_to_find = company_data.fix_days[0].get(&key).unwrap_or(&0);
                    if let Some(index) = employees.iter().position(|e| e.id == *id_to_find) {
                        lineToAdd.push_str(&employees[index].short);
                        // TODO: move this to a function, including the part in plan_employee
                        employees[index].count =
                            employees[index].count + (1.0 * (1.0 / employees[index].percent));
                        // Set last duty to this date
                        employees[index].last_duty = currentDate;
                    } else {
                        println!("Employee with ID {} not found", id_to_find);
                    }
                } else {
                    lineToAdd.push_str(&plan_employee(&mut employees, currentDate));
                }

                // Add divider of morning and afternoon (yes, it's just a coma)
                lineToAdd.push_str(",");

                // Plan afternoon employee
                let mut key = format!("{}n", currentDate.weekday().number_from_monday());
                if company_data.fix_days[0].get(&key).unwrap_or(&0) > &0 {
                    let id_to_find = company_data.fix_days[0].get(&key).unwrap_or(&0);
                    if let Some(index) = employees.iter().position(|e| e.id == *id_to_find) {
                        lineToAdd.push_str(&employees[index].short);
                        // TODO: move this to a function, including the part in plan_employee
                        employees[index].count =
                            employees[index].count + (1.0 * (1.0 / employees[index].percent));
                        // Set last duty to this date
                        employees[index].last_duty = currentDate;
                    } else {
                        println!("Employee with ID {} not found", id_to_find);
                    }
                } else {
                    lineToAdd.push_str(&plan_employee(&mut employees, currentDate));
                }

                // Add the prepared line to the output
                outputFileHandler.add_line(&lineToAdd);

                // Check for global off days
                // if (currentDate.weekday().number_from_monday() < 4) {}

                currentDate = currentDate.succ_opt().unwrap();
            }

            // Write prepared content to output file
            outputFileHandler.write_to_file("./output.csv");

            println!("Planned from {} to {}", startDate_string, endDate_string);
            println!(
                "some stuff i forgot {} to {}",
                startDate_string, endDate_string
            );

            println!("Employees planned:");
            for e in employees {
                println!(
                    "{}\t{}\t{} (effective {}) duties.",
                    e.name,
                    e.surname,
                    e.count,
                    e.count * e.percent
                );
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

fn plan_employee(employees: &mut Vec<Employee>, date: NaiveDate) -> String {
    let mut rng = thread_rng();
    employees.sort_by(|a, b| {
        a.count
            .partial_cmp(&b.count)
            .unwrap_or(std::cmp::Ordering::Greater)
    });
    let mut randomEmployeeNumber = rng.gen_range(0..employees.len() / 2);
    // let mut randomEmployeeNumber = 0;

    // println!("{}", date.format("%d.%m.%Y").to_string());

    /* Check for last planned duty and sorty by last_duty if too early in the past (2 days). */
    // TODO: make the days configurable
    if (date - employees[randomEmployeeNumber].last_duty)
        .num_days()
        .abs()
        < 2
    {
        // employees.sort_by(|a, b| a.last_duty.cmp(&b.last_duty));
        employees.sort_by(|a, b| a.last_duty.cmp(&b.last_duty));

        // randomEmployeeNumber = rng.gen_range(0..employees.len());
        randomEmployeeNumber = 0;
    }

    // Add duty to count of employee based on his working percentage
    employees[randomEmployeeNumber].count = employees[randomEmployeeNumber].count
        + (1.0 * (1.0 / employees[randomEmployeeNumber].percent));
    // Set last duty to this date
    employees[randomEmployeeNumber].last_duty = date;

    employees[randomEmployeeNumber].short.clone()
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
