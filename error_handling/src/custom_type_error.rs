// Declaring `CustomTypeError`
// Writing a `Json parser`
// Json/src/error.rs

#[derive(Debug, Clone)]
pub enum JsonError {
    pub Message: String,
    pub Line: usize,
    pub Column: usize   
}

/* 
    The correspoding return will something like this:
    return Err(JsonError {
        Message: "Invalid JSON".to_string(),
        Line: 1,
        Column: 5
    });
*/

use std::fmt; 
// In Rust, the std::fmt module provides utilities for formatting and printing text. 
//  It defines traits like Display and Debug, which are used to control how types are formatted when printed.

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write(f, "{} ( {}:{})", self.message, self.line, self.column)
    }
}

// Errors should implment the std::error::Error trait,
// but the default definitions for the Error methods are fine.
impl std::error::Error for JsonError {}
