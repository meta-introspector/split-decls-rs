use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn transform_struct (input : TokenStream) -> TokenStream { let mut parsed : syn :: ItemStruct = syn :: parse2 (input) . unwrap () ; if let syn :: Fields :: Named (ref mut fields) = parsed . fields { for field in & mut fields . named { field . attrs . push (parse_quote ! (# [debug])) ; } } quote ! (# parsed) }