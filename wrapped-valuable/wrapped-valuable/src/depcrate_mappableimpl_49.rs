// Generated macro for impl_49 (impl)
macro_rules! Depcrate_mappableimpl_49 {
() => {
// Module: crate::mappable
// Provides: {"impl_49"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < K : Valuable , V : Valuable > Valuable for alloc :: collections :: BTreeMap < K , V > { fn as_value (& self) -> Value < '_ > { Value :: Mappable (self) } fn visit (& self , visit : & mut dyn Visit) { for (key , value) in self . iter () { visit . visit_entry (key . as_value () , value . as_value ()) ; } } }
};
}
