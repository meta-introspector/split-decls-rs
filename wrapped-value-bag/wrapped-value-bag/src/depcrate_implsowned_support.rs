// Generated macro for owned_support (module)
macro_rules! Depcrate_implsowned_support {
() => {
// Module: crate::impls
// Provides: {"owned_support"}
// Dependencies: {}
# [cfg (feature = "owned")] mod owned_support { use super :: * ; use crate :: OwnedValueBag ; impl < 'v > From < & 'v OwnedValueBag > for ValueBag < 'v > { # [inline] fn from (v : & 'v OwnedValueBag) -> ValueBag < 'v > { v . by_ref () } } }
};
}
