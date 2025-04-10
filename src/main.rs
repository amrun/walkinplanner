//#![allow(warnings)]

use chrono::{Datelike, NaiveDate, Weekday};
use std::fs;
use std::io::{Write, stdin, stdout};
use std::path::Path;

mod company_data;
mod employee;
use crate::company_data::CompanyData;
use crate::employee::Employee;

mod file_handler;
use file_handler::FileHandler;

use rand::Rng;
use rand::thread_rng;

fn main() {
    let mut output_file_handler = FileHandler::new();

    // let file_path = get_file_path(String::from("input.json"));
    let file_path = "/Users/bberger/Code/walkinplanner/src/input.json";

    let mut days_planned: i128 = 0;
    let mut days_weekend_and_holidays: i128 = 0;
    let mut errors = 0;

    // Read the file and parse the content
    let result =
        read_json_file(&file_path).and_then(|json_content| parse_json_string(&json_content));

    match result {
        Ok(company_data) => {
            // Extract employees from company_data
            let mut employees = company_data.employees;
            // Parse holidays into Vec<NaiveDate>
            let global_holidays: Vec<NaiveDate> = company_data
                .global_holidays
                .iter()
                .map(|s| NaiveDate::parse_from_str(s, "%d.%m.%Y"))
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|e| {
                    println!("Failed to parse holidays: {}", e);
                    Vec::new() // Fallback to empty vec
                });

            // Parse the end date string into a NaiveDate
            let mut current_date: NaiveDate;
            let start_date_string = &company_data.from;
            match NaiveDate::parse_from_str(start_date_string, "%d.%m.%Y") {
                Ok(date) => current_date = date,
                Err(e) => {
                    println!("Error parsing end date: {}", e);
                    return;
                }
            }

            // Parse the start date string into a NaiveDate
            let end_date: NaiveDate;
            let end_date_string = &company_data.to;
            match NaiveDate::parse_from_str(end_date_string, "%d.%m.%Y") {
                Ok(date) => end_date = date,
                Err(e) => {
                    println!("Error parsing start date: {}", e);
                    return;
                }
            }

            // Add Header line to csv output
            output_file_handler.add_header_line();

            // Loop through days
            while current_date <= end_date {
                let mut line_to_add = String::new();

                // Get the german name of the weekday
                let (_number, weekday_name) = get_german_weekday(current_date.weekday());
                let current_date_and_weekday_string =
                    format_date_string(current_date) + "," + &weekday_name;

                // Check for weekends and continue if yes
                if is_weekend(current_date) {
                    current_date = current_date.succ_opt().unwrap();
                    days_weekend_and_holidays += 1;
                    continue;
                }

                // Check for global holidays
                if global_holidays.contains(&current_date) {
                    line_to_add.push_str(&current_date_and_weekday_string);
                    line_to_add.push_str(&String::from(",Feiertag,"));
                    output_file_handler.add_line(&line_to_add);
                    current_date = current_date.succ_opt().unwrap();
                    continue;
                }

                // Add current date and weekday
                line_to_add.push_str(&current_date_and_weekday_string);
                line_to_add.push_str(",");

                // Plan morning employee
                let key = format!("{}v", current_date.weekday().number_from_monday());
                if company_data.fix_days[0].get(&key).unwrap_or(&0) > &0 {
                    let id_to_find = company_data.fix_days[0].get(&key).unwrap_or(&0);
                    if let Some(index) = employees.iter().position(|e| e.id == *id_to_find) {
                        line_to_add.push_str(&employees[index].short);
                        // TODO: move this to a function, including the part in plan_employee
                        employees[index].count =
                            employees[index].count + (1.0 * (1.0 / employees[index].percent));
                        // Set last duty to this date
                        employees[index].last_duty = current_date;
                    } else {
                        println!("Employee with ID {} not found", id_to_find);
                    }
                } else {
                    let mut employee_short_to_plan = plan_employee(&mut employees, current_date);
                    let mut attempts = 0;
                    while employee_short_to_plan == "Error:isOffDay" && attempts < 5000 {
                        employee_short_to_plan = plan_employee(&mut employees, current_date);
                        attempts += 1;
                    }
                    if employee_short_to_plan.contains("Error") {
                        errors += 1;
                    }
                    line_to_add.push_str(&employee_short_to_plan);
                }

                // Add divider of morning and afternoon (yes, it's just a coma)
                line_to_add.push_str(",");

                // Plan afternoon employee
                let key = format!("{}n", current_date.weekday().number_from_monday());
                if company_data.fix_days[0].get(&key).unwrap_or(&0) > &0 {
                    let id_to_find = company_data.fix_days[0].get(&key).unwrap_or(&0);
                    if let Some(index) = employees.iter().position(|e| e.id == *id_to_find) {
                        line_to_add.push_str(&employees[index].short);
                        // TODO: move this to a function, including the part in plan_employee
                        employees[index].count =
                            employees[index].count + (1.0 * (1.0 / employees[index].percent));
                        // Set last duty to this date
                        employees[index].last_duty = current_date;
                    } else {
                        println!("Employee with ID {} not found", id_to_find);
                    }
                } else {
                    let mut employee_short_to_plan = plan_employee(&mut employees, current_date);
                    let mut attempts = 0;
                    while employee_short_to_plan == "Error:isOffDay" && attempts < 5000 {
                        employee_short_to_plan = plan_employee(&mut employees, current_date);
                        attempts += 1;
                    }
                    if employee_short_to_plan.contains("Error") {
                        errors += 1;
                    }
                    line_to_add.push_str(&employee_short_to_plan);
                }

                // Add the prepared line to the output
                output_file_handler.add_line(&line_to_add);

                current_date = current_date.succ_opt().unwrap();

                days_planned += 1;
            }

            // Write prepared content to output file
            let _ = output_file_handler.write_to_file(&get_file_path(String::from("./output.csv")));

            println!("\n\nInfo:");
            println!("------------------");
            println!("Planned from {} to {}", start_date_string, end_date_string);
            println!(
                "Weekend- and Holidays: {}",
                days_weekend_and_holidays.to_string()
            );
            println!("Days planned: {}", days_planned.to_string());
            println!("Errors: {}", errors.to_string());

            println!("\n\nEmployees planned:");
            println!("------------------");
            for e in employees {
                println!(
                    "{}\t{}\t{} duties (effective {}).",
                    e.name,
                    e.surname,
                    e.count,
                    e.count * e.percent
                );
            }

            print!("\n\nDone. Use the generated file 'output.csv' as excel import.");

            // Pause for user input
            print!("\n\nPress Enter to continue...");
            let _ = stdout().flush(); // Ensure prompt is visible
            let mut input = String::new();
            let _ = stdin().read_line(&mut input); // Wait for Enter
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
    let mut random_employee_number = rng.gen_range(0..employees.len() / 2);

    /* Check for last planned duty and sorty by last_duty if too early in the past (2 days). */
    // TODO: make the days configurable
    if (date - employees[random_employee_number].last_duty)
        .num_days()
        .abs()
        < 2
    {
        employees.sort_by(|a, b| a.last_duty.cmp(&b.last_duty));

        random_employee_number = 0;
    }

    // If the chosen Employee does not work on this day, return an error string
    let weekdaycode = format!("{}v", date.weekday().number_from_monday());
    if employees[random_employee_number]
        .off_days
        .contains(&weekdaycode)
    {
        return String::from("Error:isOffDay");
    }

    // Add duty to count of employee based on his working percentage
    employees[random_employee_number].count = employees[random_employee_number].count
        + (1.0 * (1.0 / employees[random_employee_number].percent));
    // Set last duty to this date
    employees[random_employee_number].last_duty = date;

    employees[random_employee_number].short.clone()
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

// fn get_input_path() -> Result<String, Box<dyn Error>> {
//     let exe_path = std::env::current_exe()?;
//     let dir = exe_path
//         .parent()
//         .ok_or("Couldn't get executable directory")?;
//     let path = dir.join("input.json");
//     Ok(path.to_str().ok_or("Path is not valid UTF-8")?.to_string())
// }

fn get_file_path(filename: String) -> String {
    // Get the executable path, or exit on failure
    let exe_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: Failed to get executable path: {}", e);
            std::process::exit(1);
        }
    };

    // Get the parent directory, or exit if it fails
    let dir = match exe_path.parent() {
        Some(dir) => dir,
        None => {
            eprintln!("Error: Couldn't get executable directory");
            std::process::exit(1);
        }
    };

    // Construct the full path
    let path = dir.join(filename);

    // Convert to string, or exit if not valid UTF-8
    match path.to_str() {
        Some(s) => s.to_string(),
        None => {
            eprintln!("Error: Path is not valid UTF-8");
            std::process::exit(1);
        }
    }
}
