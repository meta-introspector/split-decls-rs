use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] pub fn compress (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let data = input_str . value () ; quote ! { { let compressed = # data . split_whitespace () . collect ::< Vec < _ >> () . join (" ") ; println ! ("cargo:warning=🗜️ Compressed: {} -> {} chars" , # data . len () , compressed . len ()) ; compressed } } . into () }
}