use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Something that uniquely identifies a query invocation."] pub struct QueryInvocationId (pub u32) ;
}