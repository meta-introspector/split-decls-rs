// Generated macro for impl_257 (impl)
macro_rules! Depcrate_serdeimpl_257 {
() => {
// Module: crate::serde
// Provides: {"impl_257"}
// Dependencies: {}
# [cfg (feature = "disk")] impl Serialize for crate :: DiskKind { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let (index , variant , maybe_value) = match * self { Self :: HDD => (0 , "HDD" , None) , Self :: SSD => (1 , "SSD" , None) , Self :: Unknown (ref s) => (2 , "Unknown" , Some (s)) , } ; if let Some (ref value) = maybe_value { serializer . serialize_newtype_variant ("DiskKind" , index , variant , value) } else { serializer . serialize_unit_variant ("DiskKind" , index , variant) } } }
};
}
