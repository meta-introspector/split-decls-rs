use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] pub fn value (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let amount = input_str . value () ; quote ! { println ! ("💰 Bounty: {}" , # amount) ; } . into () }
}