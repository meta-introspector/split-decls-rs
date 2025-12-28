use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An iterator which applies a transform to the errors of the underlying"] # [doc = " iterator."] # [derive (Clone , Debug)] pub struct MapErr < I , F > { it : I , f : F , }