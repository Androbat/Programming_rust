fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16); // 10 as i16
    v.push(20i16); // 
    v
}

fn panics(){
    let mut i: i32 = 1;
    loop {
        // panic: multiplication overflowed (in any build)
        i = i.checked_mul(10).expect("multiplication overflowed")
    }
}

// fn infinity_values(){
//     // INFINITY, MAX, MIN
//     assert!((-1 / f32::INFINITY).is_sign_negative());
// }

fn checked_values_math_op(){
    let x:i32 = 300;
    let y:i32 = 200;
    // The sum of 10 and 20 can be represented as a u8
    assert_eq!(10_u8.checked_add(20), Some(30));

    // Unfortunately, the sum of 100 and 200 cannot
    assert_eq!(100_u8.checked_add(200), None);

    // Do the operation; panic if it overflows.
    let sum = x.checked_add(y).unwrap();
}

fn wrapping_operations(){
    // This product can be represented as a u16
    assert_eq!(100_u16.wrapping_mul(200), 2000)
}


fn main() {
    println!("Hello, world!");
}
