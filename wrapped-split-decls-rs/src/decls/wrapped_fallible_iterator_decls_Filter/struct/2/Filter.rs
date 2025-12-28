use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An iterator which uses a fallible predicate to determine which values of the"] # [doc = " underlying iterator should be yielded."] # [derive (Clone , Debug)] pub struct Filter < I , F > { it : I , f : F , }
}