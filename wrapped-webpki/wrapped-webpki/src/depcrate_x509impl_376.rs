// Generated macro for impl_376 (impl)
macro_rules! Depcrate_x509impl_376 {
() => {
// Module: crate::x509
// Provides: {"impl_376"}
// Dependencies: {}
impl ExtensionOid { fn lookup (id : untrusted :: Input < '_ >) -> Option < Self > { match id . as_slice_less_safe () { [first , second , x] if [* first , * second] == ID_CE => Some (Self :: Standard (* x)) , _ => None , } } }
};
}
