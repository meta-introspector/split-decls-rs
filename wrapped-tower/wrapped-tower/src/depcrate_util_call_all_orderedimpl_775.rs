// Generated macro for impl_775 (impl)
macro_rules! Depcrate_util_call_all_orderedimpl_775 {
() => {
// Module: crate::util::call_all::ordered
// Provides: {"impl_775"}
// Dependencies: {}
impl < F : Future > common :: Drive < F > for FuturesOrdered < F > { fn is_empty (& self) -> bool { FuturesOrdered :: is_empty (self) } fn push (& mut self , future : F) { FuturesOrdered :: push_back (self , future) } fn poll (& mut self , cx : & mut Context < '_ >) -> Poll < Option < F :: Output > > { Stream :: poll_next (Pin :: new (self) , cx) } }
};
}
