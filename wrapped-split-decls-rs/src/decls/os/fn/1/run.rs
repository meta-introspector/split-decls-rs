use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn run () { println ! ("cargo:warning=Running OS-specific build logic wrapper.") ; os_impl :: run () ; }