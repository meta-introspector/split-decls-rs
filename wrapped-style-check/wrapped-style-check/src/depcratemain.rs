// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let arg = env :: args () . nth (1) . unwrap_or_else (| | { eprintln ! ("Please pass a src directory as the first argument") ; std :: process :: exit (1) ; }) ; let mut bad = false ; if let Err (e) = check_directory (& Path :: new (& arg) , & mut bad) { eprintln ! ("error: {}" , e) ; std :: process :: exit (1) ; } if bad { eprintln ! ("some style checks failed") ; std :: process :: exit (1) ; } eprintln ! ("passed!") ; }
};
}
