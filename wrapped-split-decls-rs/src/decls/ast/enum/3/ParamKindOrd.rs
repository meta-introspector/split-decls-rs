use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Specifies the enforced ordering for generic parameters. In the future,"] # [doc = " if we wanted to relax this order, we could override `PartialEq` and"] # [doc = " `PartialOrd`, to allow the kinds to be unordered."] # [derive (Hash , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] pub enum ParamKindOrd { Lifetime , TypeOrConst , }
}