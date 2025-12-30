// Generated macro for impl_55 (impl)
macro_rules! Depcrate_parametersimpl_55 {
() => {
// Module: crate::parameters
// Provides: {"impl_55"}
// Dependencies: {}
impl < 'a > From < & 'a EcParameters > for AnyRef < 'a > { fn from (params : & 'a EcParameters) -> AnyRef < 'a > { match params { EcParameters :: NamedCurve (oid) => oid . into () , } } }
};
}
