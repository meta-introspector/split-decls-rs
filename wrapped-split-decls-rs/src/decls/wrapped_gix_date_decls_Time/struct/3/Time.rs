use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A timestamp with timezone."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Time { # [doc = " The seconds that have passed since UNIX epoch. This makes it UTC, or `<seconds>+0000`."] pub seconds : SecondsSinceUnixEpoch , # [doc = " The time's offset in seconds, which may be negative to match the `sign` field."] pub offset : OffsetInSeconds , }
}