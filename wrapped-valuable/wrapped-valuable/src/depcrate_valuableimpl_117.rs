// Generated macro for impl_117 (impl)
macro_rules! Depcrate_valuableimpl_117 {
() => {
// Module: crate::valuable
// Provides: {"impl_117"}
// Dependencies: {}
# [cfg (feature = "std")] impl Valuable for dyn std :: error :: Error + 'static { fn as_value (& self) -> Value < '_ > { Value :: Error (self) } fn visit (& self , visit : & mut dyn Visit) { visit . visit_value (self . as_value ()) ; } }
};
}
