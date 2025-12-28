use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [should_panic] fn out_of_order () { let mut map = SnapshotMap :: default () ; map . insert (22 , "twenty-two") ; let snapshot1 = map . snapshot () ; map . insert (33 , "thirty-three") ; let snapshot2 = map . snapshot () ; map . insert (44 , "forty-four") ; map . rollback_to (snapshot1) ; map . rollback_to (snapshot2) ; }
}