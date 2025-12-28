use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn register_functions (bundle : & mut FluentBundle) { bundle . add_function ("STREQ" , | positional , _named | match positional { [FluentValue :: String (a) , FluentValue :: String (b)] => format ! ("{}" , (a == b)) . into () , _ => FluentValue :: Error , }) . expect ("Failed to add a function to the bundle.") ; }
}