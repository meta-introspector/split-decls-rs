// Generated macro for impl_445 (impl)
macro_rules! Depcrate_deimpl_445 {
() => {
// Module: crate::de
// Provides: {"impl_445"}
// Dependencies: {}
impl < S > From < crate :: Document < S > > for Deserializer < S > { fn from (doc : crate :: Document < S >) -> Self { let crate :: Document { root , raw , .. } = doc ; let raw = Some (raw) ; Self { root , raw } } }
};
}
