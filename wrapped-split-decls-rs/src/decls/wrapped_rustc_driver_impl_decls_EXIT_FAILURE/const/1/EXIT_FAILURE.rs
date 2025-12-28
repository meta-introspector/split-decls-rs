use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " Exit status code used for compilation failures and invalid flags."] pub const EXIT_FAILURE : i32 = 1 ;
}