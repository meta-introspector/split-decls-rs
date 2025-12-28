use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone , PartialEq , Eq)] enum QueryKind { Input , Tracked , TrackedWithSalsaStruct , Transparent , Interned , }
}