use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " adapts Item of array mut reference iterator to Item of hashmap mut reference iterator."] # [inline (always)] fn adapt_array_mut_it < K , V > (pair : & mut (K , V)) -> (& K , & mut V) { let (a , b) = pair ; (a , b) }
}