use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline (always)] # [cfg (feature = "std")] fn saturating_sub_usize_u64 (a : usize , b : u64) -> usize { match usize :: try_from (b) { Ok (b) => a . saturating_sub (b) , Err (_) => 0 , } }