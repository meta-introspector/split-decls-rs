use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T , E > UnwrapCapOverflow < T > for Result < T , E > { fn unwrap_cap_overflow (self) -> T { match self { Ok (val) => val , Err (_) => capacity_overflow () , } } }