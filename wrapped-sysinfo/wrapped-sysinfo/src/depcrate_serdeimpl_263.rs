// Generated macro for impl_263 (impl)
macro_rules! Depcrate_serdeimpl_263 {
() => {
// Module: crate::serde
// Provides: {"impl_263"}
// Dependencies: {}
# [cfg (feature = "system")] impl serde :: Serialize for crate :: Product { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { let mut state = serializer . serialize_struct ("Product" , 5) ? ; state . serialize_field ("name" , & Self :: name ()) ? ; state . serialize_field ("family" , & Self :: family ()) ? ; state . serialize_field ("serial_number" , & Self :: serial_number ()) ? ; state . serialize_field ("stock_keeping_unit" , & Self :: stock_keeping_unit ()) ? ; state . serialize_field ("uuid" , & Self :: uuid ()) ? ; state . serialize_field ("version" , & Self :: version ()) ? ; state . serialize_field ("vendor_name" , & Self :: vendor_name ()) ? ; state . end () } }
};
}
