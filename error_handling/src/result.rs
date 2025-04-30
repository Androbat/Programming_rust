// RESULT -> Rust does not have exceptions. It handles the two possible scenarios,
// with the Result type. Those scenarios are:
// Success / failure.

fn get_weather(location: LatLng) -> Result<WeatherReport, io::Error>;
// The RESULT type -> Indiates a possible "FAILURE". 
// It will return the following:
// * Ok -> Success
// * Err -> Failure

// The best way to deal with a Result<T, E> is using a «Match expression»
// Example:
// match get_weather(hometown){
//  Ok(report) => {
//     display_weather(hometown, &report)
//    }
//   Err(err) => {
//     println!("error querying the weather: {}", err)
//       schedule_weather_retry();
//   }
// }

// Since match sometimes could be a little verbose,
// we got several methods, which implements «Match» under the hood,
// at our disposal to handle Result<T, E>:

// 1. result.is_ok / result.is_err()
// 2. result.ok()
// 3. result.err()
// 5. result.unwrap_or(fallback)


// RESULT TYPE ALIAS:
fn remove_file(path: &Path) -> Result<()>;
// A type alias is a kind of "shorthand" for type names.
// Modules often define a "Result" type alias to avoid having to repeat an error type

pub type Result<T> = result::Result<T, Error>;
// Defines a public type std::io::Result<T>.
// It's an alias for Result<T, E>, but hardcodes std::io::Error as the error type.

// PRINTING ERRORS 
// Standard library error printing => std::fmt::Error, std::Utf8Error, std::io::Error
// print()
// with {:?} -> you get a debug view of the error

// err.to_string() -> Error message in string
// err.source()
// writeln() -> writes the whole error from the error stream

// PROPAGATING ERRORS
// ? -> propagates the error to the stack 
fn propagate_weather_error() -> Result<WeatherReport, io::Error> {
    let report = get_weather(hometown)?; // Will return the error if any ocurrs and also wrapps the ok value.
    Ok(report) // Will return the Ok, since it is a result: Result<T, E>
    // ? Operator can only be used in Result type functions.
}

// With match operator
fn propagate_using_match() -> Result<WeatherReport, io::Error> {
    let weather = match get_weather(hometown){
        Ok(success_value) => success_value,
        Err(err) => return Err(err)
    };

    weather
}

// Example:
use std::fs;
use std::io;
use std::path::Path;

fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?;
    }

    Ok(())
}


// WORKING WITH MULTIPLE ERROR TYPES
// Reading from a file
use std::io::{self, BufRead };

fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i32>, io::Error> {
    let mut numbers = Vec![];
    for line_result in file.lines(){
        let line = line_result?;
        numbers.push(line.parse::<i32>()?);
    }

    Ok(numbers)
}

// The code above gives us an eror, which is the following:
// error: ? couldn't convert the error to `std::io::Error`
// numbers.push(line.parse()?); // parsing integers can fall 
// the trait `std::convert::From<std::num::ParseIntError>`
// is not implemented for `std::io::Error`

// Note: the question mark operation (`?`) implicitly performs a conversion
// on the error value using the `From` trait.

// Basically the error says -> std::num::ParseIntError cannot be converted to std::io::Error

// Check -> All of the standard library error types can be converted to the type 
// Box<dyn std::error::Error + Send + Sync + 'static>. -> Represents any error, where
// Send + 'static makes it safe to pass between threads
// Or use type alias such as:
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

// With the following change, we function won't break. 
// The `?` operator will convert to the type GenericError. As we know so far, 
// 
fn read_numbers_(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {}

// To convert any error just do the: GenericError::from():

fn convert() -> () {
    let io_error = io::Error::new( // Make my own io::Error
        io::ErrorKind::Other, "timed out");
    Err(GenericError::from(io_error)) // Manually converts to `GenericError`
}

// error.downcast_ref::<ErrorType>() -> Sometimes, when we're dealing with a particular case,
// we cannot use `Generic error`, since a generic error won't work, for obvious reasons.
// When that happens, we can do the following instead:
// error.downcast_ref::<ErrorType>() since it borrows a reference to the error

/* 
  * err.downcast_ref::<MissingSemicolonError>()
*/