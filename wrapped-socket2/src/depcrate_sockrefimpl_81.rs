// Generated macro for impl_81 (impl)
macro_rules! Depcrate_sockrefimpl_81 {
() => {
// Module: crate::sockref
// Provides: {"impl_81"}
// Dependencies: {}
impl < 's > Deref for SockRef < 's > { type Target = Socket ; fn deref (& self) -> & Self :: Target { & self . socket } }
};
}
