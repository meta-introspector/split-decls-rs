// Generated macro for impl_260 (impl)
macro_rules! Depcrate_serdeimpl_260 {
() => {
// Module: crate::serde
// Provides: {"impl_260"}
// Dependencies: {}
# [cfg (feature = "system")] impl Serialize for crate :: Cpu { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_struct ("Cpu" , 5) ? ; state . serialize_field ("cpu_usage" , & self . cpu_usage ()) ? ; state . serialize_field ("name" , & self . name ()) ? ; state . serialize_field ("vendor_id" , & self . vendor_id ()) ? ; state . serialize_field ("brand" , & self . brand ()) ? ; state . serialize_field ("frequency" , & self . frequency ()) ? ; state . end () } }
};
}
