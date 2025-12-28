use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Debug , Eq , PartialEq)] enum Trait { KnownLayout , Immutable , TryFromBytes , FromZeros , FromBytes , IntoBytes , Unaligned , Sized , ByteHash , ByteEq , SplitAt , }
}