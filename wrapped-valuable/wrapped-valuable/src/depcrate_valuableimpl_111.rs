// Generated macro for impl_111 (impl)
macro_rules! Depcrate_valuableimpl_111 {
() => {
// Module: crate::valuable
// Provides: {"impl_111"}
// Dependencies: {}
impl Valuable for () { fn as_value (& self) -> Value < '_ > { Value :: Tuplable (self) } fn visit (& self , visit : & mut dyn Visit) { visit . visit_unnamed_fields (& []) ; } }
};
}
