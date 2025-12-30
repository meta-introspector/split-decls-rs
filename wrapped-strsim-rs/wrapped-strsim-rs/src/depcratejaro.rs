// Generated macro for jaro (function)
macro_rules! Depcratejaro {
() => {
// Module: crate
// Provides: {"jaro"}
// Dependencies: {}
# [doc = " Calculates the Jaro similarity between two strings. The returned value"] # [doc = " is between 0.0 and 1.0 (higher value means more similar)."] # [doc = ""] # [doc = " ```"] # [doc = " use strsim::jaro;"] # [doc = ""] # [doc = " assert!((0.392 - jaro(\"Friedrich Nietzsche\", \"Jean-Paul Sartre\")).abs() <"] # [doc = "         0.001);"] # [doc = " ```"] pub fn jaro (a : & str , b : & str) -> f64 { generic_jaro (& StringWrapper (a) , & StringWrapper (b)) }
};
}
