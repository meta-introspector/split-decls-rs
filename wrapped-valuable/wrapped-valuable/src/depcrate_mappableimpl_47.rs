// Generated macro for impl_47 (impl)
macro_rules! Depcrate_mappableimpl_47 {
() => {
// Module: crate::mappable
// Provides: {"impl_47"}
// Dependencies: {}
# [cfg (feature = "std")] impl < K : Valuable , V : Valuable , S > Valuable for std :: collections :: HashMap < K , V , S > { fn as_value (& self) -> Value < '_ > { Value :: Mappable (self) } fn visit (& self , visit : & mut dyn Visit) { for (key , value) in self . iter () { visit . visit_entry (key . as_value () , value . as_value ()) ; } } }
};
}
