// Generated macro for fmt_thousands_sep (function)
macro_rules! Depcrate_benchfmt_thousands_sep {
() => {
// Module: crate::bench
// Provides: {"fmt_thousands_sep"}
// Dependencies: {}
fn fmt_thousands_sep (mut n : f64 , sep : char) -> String { use std :: fmt :: Write ; let mut output = String :: new () ; let mut trailing = false ; for & pow in & [9 , 6 , 3 , 0] { let base = 10_usize . pow (pow) ; if pow == 0 || trailing || n / base as f64 >= 1.0 { match (pow , trailing) { (0 , true) => write ! (output , "{:06.2}" , n / base as f64) . unwrap () , (0 , false) => write ! (output , "{:.2}" , n / base as f64) . unwrap () , (_ , true) => write ! (output , "{:03}" , n as usize / base) . unwrap () , _ => write ! (output , "{}" , n as usize / base) . unwrap () , } if pow != 0 { output . push (sep) ; } trailing = true ; } n %= base as f64 ; } output }
};
}
