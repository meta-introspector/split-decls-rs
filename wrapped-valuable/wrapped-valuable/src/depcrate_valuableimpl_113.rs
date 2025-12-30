// Generated macro for impl_113 (impl)
macro_rules! Depcrate_valuableimpl_113 {
() => {
// Module: crate::valuable
// Provides: {"impl_113"}
// Dependencies: {}
impl Valuable for & '_ str { fn as_value (& self) -> Value < '_ > { Value :: String (self) } fn visit (& self , visit : & mut dyn Visit) { visit . visit_value (Value :: String (self)) ; } fn visit_slice (slice : & [Self] , visit : & mut dyn Visit) where Self : Sized , { visit . visit_primitive_slice (Slice :: Str (slice)) ; } }
};
}
