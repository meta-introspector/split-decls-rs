// Generated macro for impl_359 (impl)
macro_rules! Depcrate_verify_certimpl_359 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_359"}
// Dependencies: {}
impl < 'a > PathNode < 'a > { pub (crate) fn iter (& self) -> PathIter < 'a > { PathIter { path : self . path , next : Some (self . index) , } } pub (crate) fn role (& self) -> Role { match self . index { 0 => Role :: EndEntity , _ => Role :: Issuer , } } }
};
}
