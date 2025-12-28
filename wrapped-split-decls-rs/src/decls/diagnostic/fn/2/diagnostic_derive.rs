use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn diagnostic_derive (s : Structure < '_ >) -> TokenStream { let name = & s . ast () . ident ; let expanded = s . gen_impl (quote ! { gen impl crate :: Diagnostic for @ Self { fn get_message (& self) -> String { format ! ("Diagnostic: {}" , stringify ! (# name)) } } }) ; expanded . into () }
}