mod custom_type_error;

fn main() {
    println!("Hello, world!");
}

// Ordinary errors are handled by the RESULT type. -> Result typically represent
// problems caused by things outside of the program like:
// * Error inputs
// * Network outages