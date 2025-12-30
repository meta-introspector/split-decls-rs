// Generated macro for impl_533 (impl)
macro_rules! Depcrate_itemimpl_533 {
() => {
// Module: crate::item
// Provides: {"impl_533"}
// Dependencies: {}
impl Signature { # [doc = " A method's `self` receiver, such as `&self` or `self: Box<Self>`."] pub fn receiver (& self) -> Option < & Receiver > { let arg = self . inputs . first () ? ; match arg { FnArg :: Receiver (receiver) => Some (receiver) , FnArg :: Typed (_) => None , } } }
};
}
