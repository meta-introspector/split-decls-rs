use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A `LocalDefId` is equivalent to a `DefId` with `krate == LOCAL_CRATE`. Since"] # [doc = " we encode this information in the type, we can ensure at compile time that"] # [doc = " no `DefId`s from upstream crates get thrown into the mix. There are quite a"] # [doc = " few cases where we know that only `DefId`s from the local crate are expected;"] # [doc = " a `DefId` from a different crate would signify a bug somewhere. This"] # [doc = " is when `LocalDefId` comes in handy."] # [derive (Clone , Copy , PartialEq , Eq , Hash)] pub struct LocalDefId { pub local_def_index : DefIndex , }
}