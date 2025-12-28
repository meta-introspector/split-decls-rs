use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An iterator that unwraps every element yielded by the underlying"] # [doc = " FallibleIterator"] # [derive (Clone , Debug)] pub struct Unwrap < T > (T) ;
}