use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Copy , Debug)] struct Bucket < K , V > { hash : HashValue , key : K , value : V , }
}