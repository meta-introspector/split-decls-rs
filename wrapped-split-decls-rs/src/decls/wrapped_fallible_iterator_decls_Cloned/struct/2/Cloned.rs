use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An iterator which clones the elements of the underlying iterator."] # [derive (Clone , Debug)] pub struct Cloned < I > (I) ;