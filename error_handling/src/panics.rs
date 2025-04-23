// A program panics when it encounters something so messed up that there must be a 
// bug in the program iself.

// What Rust does when a panic happens?
// Rust can either unwind the stack when a panic happens
// or abort the process. Unwind is the default. 
// -> Unwind -> Means claning the call stack where the panic happened.
fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size
}

// The code above will produce the following error:
// thread 'main' panicked at 'attempt to devide by zero', pirate.rs
// note: Run with `RUST_BACKTRACE=1` for a backtrace

// RUST_BACKTRACE=1 -> environment variables, Rust will "dump" the stack at this point.
// «Any temporary values, local variables, or arguments that the current function was using
// are dropped, in the reverse of the order they were created. Dropping value simple means
// "cleaning up after" it: any Strings or Vecs the program was using are freed.
// Any opened files are closed, and so on.»

// IMPORTANT:
// A panic is not a "crash". It is like a RuntimeException in Java.
// It's just a behavior that shoudn't be happening. 
// Panic -> it is save. Does not viaolate memory safety.
// It will never leave a "dangling pointer" behind or half initialized value in memory.

// Standard library  function -> std::panic::catch_unwind()
// It other way to handle panics and catch unwind errors.