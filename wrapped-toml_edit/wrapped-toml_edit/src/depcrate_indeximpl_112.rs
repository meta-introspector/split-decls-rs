// Generated macro for impl_112 (impl)
macro_rules! Depcrate_indeximpl_112 {
() => {
// Module: crate::index
// Provides: {"impl_112"}
// Dependencies: {}
impl < 's > ops :: Index < & 's str > for InlineTable { type Output = Value ; fn index (& self , key : & 's str) -> & Value { self . get (key) . expect ("index not found") } }
};
}
