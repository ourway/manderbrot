// Mandelbrot pattern
//
//

extern crate num;
use num::Complex;
use std::str::{FromStr};

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {

    match s.find(separator) {
    
        None => None,
        Some(index) => {

            match( T::from_str(&s[..index]), T::from_str(&s[index+1..])) {

                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }

        
}

#[test]
fn test_parse_pair(){


    assert_eq!(parse_pair::<i32>("300x400", 'x'), Some((300, 400)));
    assert_eq!(parse_pair("1.54,3.4", ','), Some((1.54, 3.4)));
    assert_eq!(parse_pair::<f64>("1.54,3.4", '|'), None);
}

//fn parse_pair_complex(s: &str) -> Option<Complex<f64>> {
//    match parse_pair::
//}
// When we have a function with None or other result type, we use `Option`
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

/// This is my documentation.
/// I want to feel free to use `it`.
/// lets see
/// ```
/// let mut wow = 1;
/// ```
///
///
fn something() {
    let hasan1: Option<u32> = escape_time(Complex { re: 0.46, im: 0.0 }, 10);
    println!("The result is {:?}\n", hasan1);
}
fn main() {
    something();
}
