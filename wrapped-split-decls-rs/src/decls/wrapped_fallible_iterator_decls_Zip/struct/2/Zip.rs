use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An iterator that yields pairs of this iterator's and another iterator's"] # [doc = " values."] # [derive (Clone , Debug)] pub struct Zip < T , U > (T , U) ;
}