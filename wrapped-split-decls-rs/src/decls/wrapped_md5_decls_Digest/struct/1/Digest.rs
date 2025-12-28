use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A digest."] # [derive (Clone , Copy , Eq , Hash , PartialEq)] pub struct Digest (pub [u8 ; 16]) ;