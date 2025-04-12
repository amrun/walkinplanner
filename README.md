# walkinplanner

Simple program for duty scheduling of a team. Highly specialized, so not for general use.
## User Manual
### Basic functionality (how it works)

1. Prepare an input file called "input.json", according to the example provided
2. Place the input file into the same folder as the executable "walkinplanner.exe"
	1. Use an otherwise empty folder
3. Execute "walkinplanner.exe"
4. Read the output provided in the shell and press enter to end the program
5. Use the generated "output.csv" file to finalize the planning in Excel
## Input file
The input file must contain valid json only. Be careful about the brackets and "," and such.
### Structure
#### General fields
- from
	- Date to start the plan (format: dd.mm.yyyy)
- to
	- Date to end the plan (format: dd.mm.yyyy)
- globalOffDays **(not implemented yet!)**
	- Weekdays on which NOT to plan any duty
	- Saturday and Sunday are not planned by default
	- The format is:
		- Monday morning: 1v
		- Monday afternoon: 1n
		- Tuesday morning: 2v
		- Tuesday afternoon: 2v
		- etc.
- globalHolidays
	- Dates on which not to plan any duty (format: dd.mm.yyyy)
	- The entry will be "Ferientag" for these dates
- fix_days
	- Days on which to ALWAYS plan one 
	- This superseds the off days of employees
	- Enter the **id** of the employee, of which to plan by default
		- 0 for random employee
- employees
	- Copy and edit as many employee blocks as needed
	- Fields:
		- id: ID of the employee. **Must be unique**
		- count: leave as 0.0
		- name: Name of the employee
		- surname: Surname of the employee
		- short: Abbreviation of the employee (eg. "JD" for "John Doe")
			- Can be as many characters as needed
			- Is used to mark which employee is planned on which day
		- percent: Employment percentage
			- eg. 0.8 for 80%, 1.0 for 100%, etc.
			- This modifies the duty-counter (employees with lower percentage have less duties)
		- off_days: Weekdays on which the employee is not working (eg. 80% and not working on fridays -> "5v,5n")
		- last_duty: leave as 01.01.1970
