use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Helper trait for indexing operations with a custom index type."] pub trait IntoSliceIdx < I , T : ? Sized > { type Output : SliceIndex < T > ; fn into_slice_idx (self) -> Self :: Output ; }
}