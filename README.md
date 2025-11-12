# rustoleum

A Rust library and CLI tool for unit conversion between temperature and volume units.

This project provides both a command-line interface for instructors to verify student answers on Unit Conversion worksheets, and a library API for programmatic use in Rust applications.

## Features

- **Type-safe API**: Uses Rust enums (`Unit`) instead of strings for compile-time safety
- **Comprehensive conversions**: Supports all conversions between:
  - **Temperature units**: Kelvin, Celsius, Fahrenheit, and Rankine
  - **Volume units**: Liters, Tablespoons, Cubic Inches, Cups, Cubic Feet, and Gallons
- **Well-documented**: Full API documentation with examples
- **Well-tested**: 51 unit tests covering all conversion paths
- **Library API**: Can be used as a dependency in other Rust projects
- **CLI tool**: Ready-to-use command-line interface for grading worksheets

## Supported Units

### Temperature
- Kelvin (absolute temperature scale)
- Celsius (metric temperature scale)
- Fahrenheit (imperial temperature scale)
- Rankine (absolute temperature scale, Fahrenheit-based)

### Volume
- Liters (metric)
- Tablespoons (US customary)
- Cubic Inches (imperial)
- Cups (US customary)
- Cubic Feet (imperial)
- Gallons (US customary)

## Requirements
1. The teacher must be able to provide an input numerical value, an input unit of measure, a target
unit of measure, and a student’s numeric response.
2. The system indicates that the response is **correct**, **incorrect**, or **invalid**. To be
considered **correct**, the student’s response must match an authoritative answer after both the
student’s response and authoritative answer are rounded to the *tenths* place.

## Installation

### Docker (Recommended for CLI)

The pre-built Docker image is available on Docker Hub:

```bash
docker pull l0r3zz/rustoleum:latest
```

### Building from Source

Requires Rust 1.91 or later.

```bash
# Clone the repository
git clone https://github.com/l0r3zz/rustoleum.git
cd rustoleum

# Build the project
cargo build --release

# The binary will be at target/release/rustoleum
```

### As a Library Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
rustoleum = "0.2.0"
```
## Usage

### CLI Usage

The command-line interface accepts four arguments:

```
USAGE:
    rustoleum <input units> <target units> <control> <answer>
```

**Arguments:**
- `<input units>` - The source unit of measure (case-insensitive)
- `<target units>` - The target unit of measure (case-insensitive)
- `<control>` - The numerical value to be converted
- `<answer>` - The student's answer to verify

**Output:**
- `Answer: correct` - The student's answer matches the expected conversion
- `Answer: incorrect` - The student's answer does not match
- `Answer: invalid` - Invalid units or conversion (e.g., temperature to volume)

**Examples:**

```bash
# Using Docker
docker run l0r3zz/rustoleum:latest rustoleum celsius kelvin 70.0 343.15
Answer: correct

# Using local binary
rustoleum kelvin fahrenheit 100 -279.67
Answer: correct

rustoleum fahrenheit celsius 70.0 21.0
Answer: incorrect

rustoleum kelvin dog 100 -279.67
Answer: invalid

rustoleum celsius liters 100 50.0
Answer: invalid
```

### Library Usage

The library provides a type-safe API for unit conversions:

```rust
use rustoleum::{Unit, convert};

// Convert 70 degrees Celsius to Kelvin
let result = convert(Unit::Celsius, Unit::Kelvin, 70.0);
assert_eq!(result, Some(343.15));

// Convert 1 liter to gallons
let result = convert(Unit::Liters, Unit::Gallons, 1.0);
assert_eq!(result, Some(0.2641));

// Parse units from strings
use std::str::FromStr;
let unit = Unit::from_str("celsius").unwrap();
assert_eq!(unit, Unit::Celsius);

// Invalid conversions return None
let result = convert(Unit::Celsius, Unit::Liters, 100.0);
assert_eq!(result, None);
```

For more examples and complete API documentation, see the [generated documentation](https://docs.rs/rustoleum) or build it locally:

```bash
cargo doc --open
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run only library tests
cargo test --lib
```

### Building Documentation

```bash
# Generate and open documentation
cargo doc --open

# Generate documentation without opening
cargo doc --no-deps
```

### Code Quality

The project uses:
- **Rust Edition**: 2021
- **Clippy**: For linting (run with `cargo clippy`)
- **Rustfmt**: For code formatting (run with `cargo fmt`)

## Planned Enhancements

1. ✅ ~~Build out more comprehensive test coverage~~ - **Completed**: 51 tests covering all conversions
2. Better command line parsing (using `clap` or similar)
3. Emit JSON output to stdout and accept JSON input to stdin for automation/webhook integration
4. Add additional conversion pairs (e.g., length, weight, etc.)
5. Create a web-based solution using React with this artifact as a core

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]
