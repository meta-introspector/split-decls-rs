use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An iterator which yields elements of the underlying iterator in reverse"] # [doc = " order."] # [derive (Clone , Debug)] pub struct Rev < I > (I) ;
}