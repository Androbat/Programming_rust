// Dealing with errors that cannot happen
// digits.parse::<u64>(); -> Parses a string of digits. It returns a `Result`.
// The most common error that can occur with this is having to parse a <str> that's
// not a digit

// .unwrap -> a method that panics if the result is an `Err`, but
// returns the success value of an Ok:

// let num = digits.parse::<u64>().unwrap(); -> it works similar to `?`, but
// instead of returning the error, it will panic. Besides that another one can happen.
// "999999999999999".parse::<u64>() // overflow error.
// The u64 won't be enough to store the digit above.

// Handling errors in main()
// Normally, main cannot use `?` because its return type is not Result.
// So the simplest way is to use .expect().
// Panicking in the main thread prints an error message and then exits with
// nonzero exit grams.

/*
    fn main(){
        calculate_tides().expect()

    }
*/

/*
    fn main(){
        if let Err(err) = calculate_tides(){
            print_error(&err);
            std::process::exit(1);
        }
    }

*/
