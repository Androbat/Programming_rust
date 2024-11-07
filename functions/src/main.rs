fn main() {
    
}

fn greatest_common_factor(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}


// Just run with cargo test command
#[test]
fn test_greatest_common_factor(){

    assert_eq!(greatest_common_factor(14, 15), 1);

    let factor1 = 2 * 3 * 5 * 11 * 17;
    let factor2 = 3 * 7 * 11 * 13 * 19;
    assert_eq!(greatest_common_factor(factor1, factor2), 3 * 11);
}
