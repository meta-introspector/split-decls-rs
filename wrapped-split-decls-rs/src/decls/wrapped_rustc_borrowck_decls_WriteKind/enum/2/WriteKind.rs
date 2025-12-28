use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Kind of write access to a value"] # [doc = " (For informational purposes only)"] # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum WriteKind { StorageDeadOrDrop , Replace , MutableBorrow (BorrowKind) , Mutate , Move , }
}