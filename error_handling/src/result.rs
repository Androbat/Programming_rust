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


