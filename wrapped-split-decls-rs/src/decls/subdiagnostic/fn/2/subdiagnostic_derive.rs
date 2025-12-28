use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn subdiagnostic_derive (s : Structure < '_ >) -> TokenStream { let name = & s . ast () . ident ; let expanded = s . gen_impl (quote ! { gen impl crate :: Subdiagnostic for @ Self { fn get_message (& self) -> String { format ! ("Subdiagnostic: {}" , stringify ! (# name)) } } }) ; expanded . into () }