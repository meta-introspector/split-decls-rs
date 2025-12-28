use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [cfg_attr (all (feature = "gecko-ffi" , any (test , miri)) , repr (align (8)))] # [repr (C)] struct Header { _len : SizeType , _cap : SizeType , }
}