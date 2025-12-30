// Generated macro for impl_129 (impl)
macro_rules! Depcrate_hygieneimpl_129 {
() => {
// Module: crate::hygiene
// Provides: {"impl_129"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for LocalExpnId { fn encode (& self , e : & mut E) { self . to_expn_id () . encode (e) ; } }
};
}
