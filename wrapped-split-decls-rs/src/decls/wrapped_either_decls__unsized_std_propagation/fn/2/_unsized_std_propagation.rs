use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "std")] fn _unsized_std_propagation () { check_t ! (:: std :: path :: Path) ; check_t ! (:: std :: ffi :: OsStr) ; check_t ! (:: std :: ffi :: CStr) ; }