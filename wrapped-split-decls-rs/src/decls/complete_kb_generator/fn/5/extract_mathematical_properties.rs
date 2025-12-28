use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: extract_mathematical_properties");
fn extract_mathematical_properties (symbol : & str , node : & serde_json :: Value) -> Vec < String > { let mut properties = Vec :: new () ; let integers = extract_integers_from_symbol (symbol) ; for int in integers { if int > 0 && (int & (int - 1)) == 0 { properties . push (format ! ("lmdfb:usesPowerOfTwo {}" , int)) ; } if is_prime (int . abs ()) { properties . push (format ! ("lmdfb:usesPrime {}" , int)) ; } if int == 25519 || int == - 25519 { properties . push ("lmdfb:usesCurve25519Constant true" . to_string ()) ; } } if let Some (complexity) = node . get ("complexity_score") . and_then (| v | v . as_f64 ()) { if complexity > 10.0 { properties . push ("lmdfb:hasHighComplexity true" . to_string ()) ; } } properties }
}