use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn main () { let args : Vec < String > = std :: env :: args () . collect () ; if args . len () < 2 { eprintln ! ("Usage: {} <rust_code>" , args [0]) ; std :: process :: exit (1) ; } let input_code = & args [1] ; let tokens = match input_code . parse :: < TokenStream > () { Ok (t) => t , Err (e) => { eprintln ! ("Parse error: {}" , e) ; std :: process :: exit (1) ; } } ; let generator = BottMacroGenerator :: new (tokens) ; let output = generator . generate_all_levels () ; println ! ("{}" , output) ; }
}