use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: resume_unwinding");
pub (super) fn resume_unwinding (payload : Box < dyn Any + Send >) -> ! { panic :: resume_unwind (payload) }
}