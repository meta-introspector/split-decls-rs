// Generated macro for impl_116 (impl)
macro_rules! Depcrate_valuableimpl_116 {
() => {
// Module: crate::valuable
// Provides: {"impl_116"}
// Dependencies: {}
# [cfg (feature = "std")] impl Valuable for std :: path :: PathBuf { fn as_value (& self) -> Value < '_ > { Value :: Path (self) } fn visit (& self , visit : & mut dyn Visit) { visit . visit_value (Value :: Path (self)) ; } }
};
}
