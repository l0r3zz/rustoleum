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
USAGE:
    rustoleum <input units> <target units> <control> <answer>
    
l0r3zz@tarnover$ ddocker run  l0r3zz/rustoleum:latest rustoleum celsius kelvin 70.0 343.15 
Answer: correct
```
Where:
* `<control>` the proctor supplied numerical value to be converted
* `<answer>` the student supplied answer to the problem
*  `<input units>` the option tag followed by the accepted named unit of measure to be converted
*  `<target units>` the option tag followed by the target accepted unit of measure


Examples:
```
l0r3zz@tarnover$ rustoleum kelvin fahrenheit 100 -279.67
Answer: correct
l0r3zz@tarnover$ rustoleum fahrenheit celsius 70.0 21.0 
Answer: incorrect
l0r3zz@tarnover$ rustoleum kelvin dog 100 -279.67
Answer: invalid
l0r3zz@tarnover$ rustoleum elvin fahrenheit 100 -cat.67
Answer: invalid
```

## Planned enhancements
1. Build out more comprehensive test coverage 
2. Better command line parsing
3. Emit JSON output to stdout and accept JSON input to stdin so the tool can be used in automation or via webhook
4. Add additional conversion pairs
5. Create a web based solution using REACT with this artifact as a core
