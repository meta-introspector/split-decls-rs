// Generated macro for impl_115 (impl)
macro_rules! Depcrate_valuableimpl_115 {
() => {
// Module: crate::valuable
// Provides: {"impl_115"}
// Dependencies: {}
# [cfg (feature = "std")] impl Valuable for & std :: path :: Path { fn as_value (& self) -> Value < '_ > { Value :: Path (self) } fn visit (& self , visit : & mut dyn Visit) { visit . visit_value (Value :: Path (self)) ; } }
};
}
