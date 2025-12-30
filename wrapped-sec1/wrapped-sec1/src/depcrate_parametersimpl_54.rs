// Generated macro for impl_54 (impl)
macro_rules! Depcrate_parametersimpl_54 {
() => {
// Module: crate::parameters
// Provides: {"impl_54"}
// Dependencies: {}
impl EcParameters { # [doc = " Obtain the `namedCurve` OID."] pub fn named_curve (self) -> Option < ObjectIdentifier > { match self { Self :: NamedCurve (oid) => Some (oid) , } } }
};
}
