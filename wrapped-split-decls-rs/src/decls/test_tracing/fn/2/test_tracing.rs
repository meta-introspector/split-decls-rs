use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_tracing");
pub fn test_tracing () -> Result < () > { println ! ("🧪 Testing function call tracing...") ; let result = test_traced_function ("Bootstrap3") ? ; println ! ("Result: {}" , result) ; println ! ("✅ Tracing test completed!") ; Ok (()) }
}