use std::str::FromStr;

use num::Complex;

/// Parse the string `s` as a coordiante pair
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    // Optiton<(T, T)> -> Type for a tuple of two values
    match s.find(separator) {
        None => None,
        Some(index) => {
            // Tuple expression that parses two values to
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                // Tuple of results taking every value in it,
                // taking Ok(l) and Ok(r) as the two values in the tuple that are going to be validated.
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

// Converts from image to complex number
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {

    // Structure with two values
    // lower_right.re - upper_left.re will be saved in width
    // and the other operation in the other one.
    let (width, height) = (
        lower_right.re - upper_left.re,        
        upper_left.im - lower_right.im,        
    );
    
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}


// Parse pair test
#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10", ','), None);
    assert_eq!(parse_pair::<i32>("10, 20", ','), Some((10, 20)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25, -0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    )
}


#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),                          // Bounds: (width, height)
            (25, 175),                           // Pixel position
            Complex { re: -1.0, im: 1.0 },       // Upper-left corner
            Complex { re: 1.0, im: -1.0 },       // Lower-right corner
        ),
        Complex { re: -0.5, im: -0.75 }         // Expected result
    );
}


