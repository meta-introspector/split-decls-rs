// Generated macro for impl_144 (impl)
macro_rules! Depcrate_propagate_headerimpl_144 {
() => {
// Module: crate::propagate_header
// Provides: {"impl_144"}
// Dependencies: {}
impl < S > Layer < S > for PropagateHeaderLayer { type Service = PropagateHeader < S > ; fn layer (& self , inner : S) -> Self :: Service { PropagateHeader { inner , header : self . header . clone () , } } }
};
}
