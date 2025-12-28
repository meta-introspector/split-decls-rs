use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclunion! {
union ModeUnion { # [doc = " Indicates if the cell is locked. Only used if `Lock.mode` is `NoSync`."] no_sync : ManuallyDrop < Cell < bool > > , # [doc = " A lock implementation that's only used if `Lock.mode` is `Sync`."] sync : ManuallyDrop < RawMutex > , }
}