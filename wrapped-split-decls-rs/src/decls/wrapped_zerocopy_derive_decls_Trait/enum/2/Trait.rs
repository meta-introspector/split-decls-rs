use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Debug , Eq , PartialEq)] enum Trait { KnownLayout , Immutable , TryFromBytes , FromZeros , FromBytes , IntoBytes , Unaligned , Sized , ByteHash , ByteEq , SplitAt , }