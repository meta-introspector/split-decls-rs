use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: repeat_err");
# [doc = " Creates an iterator that endlessly repeats a single error."] pub fn repeat_err < T , E : Clone > (value : E) -> RepeatErr < T , E > { RepeatErr (PhantomData , value) }
}