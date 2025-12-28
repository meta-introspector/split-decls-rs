use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Copy , PartialEq , Eq)] pub enum Access { Shared , Exclusive , Owned , }
}