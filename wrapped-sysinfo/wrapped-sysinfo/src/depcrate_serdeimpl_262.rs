// Generated macro for impl_262 (impl)
macro_rules! Depcrate_serdeimpl_262 {
() => {
// Module: crate::serde
// Provides: {"impl_262"}
// Dependencies: {}
# [cfg (feature = "system")] impl serde :: Serialize for crate :: Motherboard { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { let mut state = serializer . serialize_struct ("Motherboard" , 5) ? ; state . serialize_field ("name" , & self . name ()) ? ; state . serialize_field ("vendor_name" , & self . vendor_name ()) ? ; state . serialize_field ("version" , & self . version ()) ? ; state . serialize_field ("serial_number" , & self . serial_number ()) ? ; state . serialize_field ("asset_tag" , & self . asset_tag ()) ? ; state . end () } }
};
}
