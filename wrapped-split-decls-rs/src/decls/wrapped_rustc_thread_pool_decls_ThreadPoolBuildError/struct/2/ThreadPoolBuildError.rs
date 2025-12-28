use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Error when initializing a thread pool."] # [derive (Debug)] pub struct ThreadPoolBuildError { kind : ErrorKind , }
}