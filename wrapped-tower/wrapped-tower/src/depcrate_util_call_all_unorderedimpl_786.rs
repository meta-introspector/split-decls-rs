// Generated macro for impl_786 (impl)
macro_rules! Depcrate_util_call_all_unorderedimpl_786 {
() => {
// Module: crate::util::call_all::unordered
// Provides: {"impl_786"}
// Dependencies: {}
impl < F : Future > common :: Drive < F > for FuturesUnordered < F > { fn is_empty (& self) -> bool { FuturesUnordered :: is_empty (self) } fn push (& mut self , future : F) { FuturesUnordered :: push (self , future) } fn poll (& mut self , cx : & mut Context < '_ >) -> Poll < Option < F :: Output > > { Stream :: poll_next (Pin :: new (self) , cx) } }
};
}
