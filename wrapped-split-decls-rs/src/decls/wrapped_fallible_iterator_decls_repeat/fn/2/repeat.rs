use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: repeat");
# [doc = " Creates an iterator that endlessly repeats a single element."] pub fn repeat < T : Clone , E > (value : T) -> Repeat < T , E > { Repeat (value , PhantomData) }
}