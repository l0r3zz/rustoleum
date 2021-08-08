# rustoleum
## Rust CLI program to process science class test scores

This is a simple CLI tool that can be run by an instructor to verify student answers on Unit Conversion worksheets.

After students turn in their completed worksheets, the teachers want to be able to
enter the questions and student responses into a computer to be graded. Students will convert:
* `Temperatures` between *Kelvin, Celsius, Fahrenheit, and Rankine*
* `Volumes` between *liters, tablespoons, cubic-inches, cups, cubic-feet, and gallons*

## Requirements
1. The teacher must be able to provide an input numerical value, an input unit of measure, a target
unit of measure, and a student’s numeric response.
2. The system indicates that the response is **correct**, **incorrect**, or **invalid**. To be
considered **correct**, the student’s response must match an authoritative answer after both the
student’s response and authoritative answer are rounded to the *tenths* place.

## Installation
The image resides on dockerhub.com 
```
l0r3zz@tarnover$ docker pull  l0r3zz/rustoleum:latest
```
## Usage

```
l0r3zz@tarnover$ docker run  l0r3zz/rustoleum:latest rustoleum
error: The following required arguments were not provided:
    <control>
    <answer>

USAGE:
    rustoleum <control> <answer> --uom_in <uom-in> --uom_target <uom-target>
    
l0r3zz@tarnover$ ddocker run  l0r3zz/rustoleum:latest rustoleum 70.0 343.15 --uom_in celsius -t kelvin
Value for uom_in: CELSIUS
Value for uom_target: KELVIN
Value for control: 70.0
Value for answer: 343.15
Answer: correct
```
Where:
* `<control>` the proctor supplied numerical value to be converted
* `<answer>` the student supplied answer to the problem
*  `--uom_in <uom_in>` the option tag followed by the accepted named unit of measure to be converted
*  `--uom_target <uom_target>` the option tag followed by the target accepted unit of measure

Examples:
```
l0r3zz@tarnover$ rustoleum 70.0 343.15 --uom_in celsius -t kelvin
Value for uom_in: CELSIUS
Value for uom_target: KELVIN
Value for control: 70.0
Value for answer: 343.15
Answer: correct
```
```
l0r3zz@tarnover$ rustoleum 70.0 21.0 --uom_in fahrenheit -t celsius
Value for uom_in: FAHRENHEIT
Value for uom_target: CELSIUS
Value for control: 70.0
Value for answer: 21.0
Answer: incorrect
```

## Planned enhancements
1. Build out more comprehensive test coverage 
2. Refactor the code so that the `<control>` and `<answer>` values can be entered at the end of the command line (speeds up usage)
3. Refactor so that the verbose information is behind a *--verbose* flag.  In other words output would just be *correct* , *incorrect*, or *invalid*
3. Emit JSON output to stdout and accept JSON input to stdin so the tool can be used in automation or via webhook
5. Add additional conversion pairs
6. Create a web based solution using REACT with this artifact as a core
