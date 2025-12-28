use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy , PartialEq , Eq)] pub enum Access { Shared , Exclusive , Owned , }