use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Error when initializing a thread pool."] # [derive (Debug)] pub struct ThreadPoolBuildError { kind : ErrorKind , }