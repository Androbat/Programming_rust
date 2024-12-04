
struct Tuple <T>(T);

pub fn heap() {
    let t: Box<(i32, &str)> = (12, "egg"); 
    
    // Allows to allocate values in the HEAP.
    // When we call Box::new() -> it allocate
    // whatever it's  in the t, in HEAP.
    let b = Box::new(t);
}

// Rust has 3 ways to represent sequence of values: ARRAYS, VECTORS and SLICES

// ARRAYS: type [T; N] you cannot append values to arrays.
// VECTORS: type Vec<T>. Vector elements lives in the HEAP.
// SLICES: type &[T]

// Arrays
fn arrays(){
    // Type of the elements in the array, u32 for this case
    // number of elements it has.
    let lazy_caterer = [u32; 6] = [1, 2, 3, 4, 5, 6];
    let taxonomy = ["Animalia", "Insecta"];

    
}


// A Vec<T> consist of 3 values:
// 1. A pointer to the heap-allocated buffer for the elements. Owned by Vec<T>
// 2. The number of elements that the buffer has the capacity to store
// 3. The number it actually contains at the moment (its length).

// Vec::with_capacity - works in the following way:
// Set an inicial capacity for the vector. If by any chance,
// the vector grows more. The vector just will increment its capacaity as usual.
fn vector(){
    // Build a vector from the values produce by an iterator
    // You will aftenly have to supply the type when using collect()
    // Since collect will need to know to what data structure convert the values to.
    let v: Vec<i32> = (0..5).collect();

    // Reverse 
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();

    // You can insert and remove elements from a vector,
    // with the case that removing an element returns a result 
    // which considers two cases:
    // - None if the vector is empty.
    // - Some to remove the element you want to.
}

fn slices(){
    // A "SLICE" written [T] without spacifying the lengh, is a region of an array of vector.
    // Since slices can be any length, slices can't be stored directly in variables
    // or passed as function arguments. Slices are always passed by reference.

    // A REFERENCE to a slice is a FAT POINTER. Something like this: [f64; 2],
    // where the first value is a pointer to the first element and the number of elements
    // in the SLICE.
    let v: Vec<f64> = vec![0.0, 0.707];
    let a: [f64; 2];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

}

// STILL SLICES
fn print(n: &[f64]){
    let v: Vec<f64> = vec![0.0, 0.707];
    let a: [f64; 2] = [0.0, 0.707];

    for elt in n {
        println!("{}", elt);
    }
}

// print(&a) -> Works on arrays.
// print(&v) -> Works on vectors.


// You can get a reference to a slice of an array or vector
// by indexing it with the range
// EXAMPLE:
// print(&v[0..2]) -> Gets 1st and 3rd elements from the array or vector
// print(&[2..])

fn strings(){
    // BYTE strings
    let method = b"GET"; // Reference to an array of 3 BYTES
}

