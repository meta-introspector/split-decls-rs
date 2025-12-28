use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (feature = "std")] impl TryFrom < std :: vec :: Vec < u8 > > for Uuid { type Error = Error ; fn try_from (value : std :: vec :: Vec < u8 >) -> Result < Self , Self :: Error > { Uuid :: from_slice (& value) } }
}