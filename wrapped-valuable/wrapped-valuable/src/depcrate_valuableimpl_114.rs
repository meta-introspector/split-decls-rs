// Generated macro for impl_114 (impl)
macro_rules! Depcrate_valuableimpl_114 {
() => {
// Module: crate::valuable
// Provides: {"impl_114"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Valuable for alloc :: string :: String { fn as_value (& self) -> Value < '_ > { Value :: String (& self [..]) } fn visit (& self , visit : & mut dyn Visit) { visit . visit_value (Value :: String (self)) ; } fn visit_slice (slice : & [Self] , visit : & mut dyn Visit) where Self : Sized , { visit . visit_primitive_slice (Slice :: String (slice)) ; } }
};
}
