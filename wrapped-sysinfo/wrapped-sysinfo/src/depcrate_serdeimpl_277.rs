// Generated macro for impl_277 (impl)
macro_rules! Depcrate_serdeimpl_277 {
() => {
// Module: crate::serde
// Provides: {"impl_277"}
// Dependencies: {}
# [cfg (feature = "user")] impl Serialize for crate :: User { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_struct ("User" , 4) ? ; state . serialize_field ("id" , & self . id ()) ? ; state . serialize_field ("group_id" , & self . group_id ()) ? ; state . serialize_field ("name" , & self . name ()) ? ; state . serialize_field ("groups" , & self . groups ()) ? ; state . end () } }
};
}
