use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [diagnostic :: on_unimplemented (message = "`{Self}` doesn't implement `DynSync`. \
            Add it to `rustc_data_structures::marker` or use `IntoDynSyncSend` if it's already `Sync`")] pub unsafe auto trait DynSync { }
}