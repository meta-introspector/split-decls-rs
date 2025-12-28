use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn try_and_fail () { let res = try_slice_owned (vec ! [0] , | v | v . get (12 ..) . ok_or (())) ; assert ! (res . is_err ()) ; }
}