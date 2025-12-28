use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A kind of wide character encoding."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum WideEncoding { # [doc = " UTF-16."] Utf16 , # [doc = " UTF-32."] Utf32 , }