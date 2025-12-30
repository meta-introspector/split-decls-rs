// Generated macro for impl_804 (impl)
macro_rules! Depcrate_specimpl_804 {
() => {
// Module: crate::spec
// Provides: {"impl_804"}
// Dependencies: {}
impl < S : Encoder > Encodable < S > for TargetTuple { fn encode (& self , s : & mut S) { match self { TargetTuple :: TargetTuple (tuple) => { s . emit_u8 (0) ; s . emit_str (tuple) ; } TargetTuple :: TargetJson { path_for_rustdoc : _ , tuple , contents } => { s . emit_u8 (1) ; s . emit_str (tuple) ; s . emit_str (contents) ; } } } }
};
}
