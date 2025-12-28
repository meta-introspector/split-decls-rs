use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: string_from_literal");
fn string_from_literal (literal : Literal) -> String { let string_literal = literal . to_string () ; if ! string_literal . starts_with ('\"') || ! string_literal . ends_with ('\"') { panic ! ("Expected a string literal, got '{}'" , string_literal) ; } string_literal [1 .. string_literal . len () - 1] . to_string () }
}