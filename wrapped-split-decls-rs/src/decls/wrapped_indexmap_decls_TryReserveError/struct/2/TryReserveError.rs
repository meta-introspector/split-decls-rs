use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " The error type for [`try_reserve`][IndexMap::try_reserve] methods."] # [derive (Clone , PartialEq , Eq , Debug)] pub struct TryReserveError { kind : TryReserveErrorKind , }
}