use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "alloc")] # [inline (always)] fn checked_min (one : Option < usize > , other : Option < usize >) -> Option < usize > { if let Some (a) = one { if let Some (b) = other { Some (:: core :: cmp :: min (a , b)) } else { Some (a) } } else { other } }