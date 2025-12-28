use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Types which consume data with byte granularity."] pub trait Update { # [doc = " Update state using the provided data."] fn update (& mut self , data : & [u8]) ; # [doc = " Digest input data in a chained manner."] # [must_use] fn chain (mut self , data : impl AsRef < [u8] >) -> Self where Self : Sized , { self . update (data . as_ref ()) ; self } }