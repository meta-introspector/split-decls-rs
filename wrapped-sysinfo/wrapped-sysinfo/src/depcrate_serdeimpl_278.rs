// Generated macro for impl_278 (impl)
macro_rules! Depcrate_serdeimpl_278 {
() => {
// Module: crate::serde
// Provides: {"impl_278"}
// Dependencies: {}
# [cfg (feature = "user")] impl Serialize for crate :: Group { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_struct ("Group" , 2) ? ; state . serialize_field ("id" , & self . id ()) ? ; state . serialize_field ("name" , & self . name ()) ? ; state . end () } }
};
}
