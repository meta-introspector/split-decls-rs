// Generated macro for impl_545 (impl)
macro_rules! Depcrate_itemimpl_545 {
() => {
// Module: crate::item
// Provides: {"impl_545"}
// Dependencies: {}
impl Signature { # [doc = " A method's `self` receiver, such as `&self` or `self: Box<Self>`."] pub fn receiver (& self) -> Option < & Receiver > { let arg = self . inputs . first () ? ; match arg { FnArg :: Receiver (receiver) => Some (receiver) , FnArg :: Typed (_) => None , } } }
};
}
