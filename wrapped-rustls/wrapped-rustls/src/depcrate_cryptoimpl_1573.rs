// Generated macro for impl_1573 (impl)
macro_rules! Depcrate_cryptoimpl_1573 {
() => {
// Module: crate::crypto
// Provides: {"impl_1573"}
// Dependencies: {}
impl Deref for StartedKeyExchange { type Target = dyn ActiveKeyExchange ; fn deref (& self) -> & Self :: Target { match self { Self :: Single (s) => s . as_ref () , Self :: Hybrid (h) => h . as_key_exchange () , } } }
};
}
