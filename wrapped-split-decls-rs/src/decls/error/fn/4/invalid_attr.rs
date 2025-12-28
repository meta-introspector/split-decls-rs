use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: invalid_attr");
# [doc = " Returns an error diagnostic for an invalid attribute."] pub (crate) fn invalid_attr (attr : & Attribute) -> Diagnostic { let span = attr . span () . unwrap () ; let path = path_to_string (attr . path ()) ; match attr . meta { Meta :: Path (_) => span_err (span , format ! ("`#[{path}]` is not a valid attribute")) , Meta :: NameValue (_) => span_err (span , format ! ("`#[{path} = ...]` is not a valid attribute")) , Meta :: List (_) => span_err (span , format ! ("`#[{path}(...)]` is not a valid attribute")) , } }
}