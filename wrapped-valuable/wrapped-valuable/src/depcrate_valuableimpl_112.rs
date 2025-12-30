// Generated macro for impl_112 (impl)
macro_rules! Depcrate_valuableimpl_112 {
() => {
// Module: crate::valuable
// Provides: {"impl_112"}
// Dependencies: {}
impl < T : Valuable > Valuable for Option < T > { fn as_value (& self) -> Value < '_ > { match self { Some (v) => v . as_value () , None => Value :: Unit , } } fn visit (& self , visit : & mut dyn Visit) { visit . visit_value (self . as_value ()) ; } }
};
}
