// Generated macro for impl_83 (impl)
macro_rules! Depcrate_log_supportimpl_83 {
() => {
// Module: crate::log_support
// Provides: {"impl_83"}
// Dependencies: {}
impl < 'a > Serialize for SerializeEventFieldMapStrippingLog < 'a > { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { let len = self . 0 . fields () . count () ; let serializer = serializer . serialize_map (Some (len)) ? ; let mut visitor = SerdeMapVisitorStrippingLog :: new (serializer) ; self . 0 . record (& mut visitor) ; visitor . finish () } }
};
}
