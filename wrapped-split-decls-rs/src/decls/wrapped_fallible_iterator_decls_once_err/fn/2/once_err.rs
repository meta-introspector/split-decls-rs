use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: once_err");
# [doc = " Creates an iterator that fails with a predetermined error exactly once."] pub fn once_err < T , E > (value : E) -> OnceErr < T , E > { OnceErr (PhantomData , Some (value)) }
}