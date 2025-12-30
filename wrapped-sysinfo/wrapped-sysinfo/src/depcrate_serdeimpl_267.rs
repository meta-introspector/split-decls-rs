// Generated macro for impl_267 (impl)
macro_rules! Depcrate_serdeimpl_267 {
() => {
// Module: crate::serde
// Provides: {"impl_267"}
// Dependencies: {}
# [cfg (feature = "system")] impl Serialize for crate :: LoadAvg { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_struct ("LoadAvg" , 3) ? ; state . serialize_field ("one" , & self . one) ? ; state . serialize_field ("five" , & self . five) ? ; state . serialize_field ("fifteen" , & self . fifteen) ? ; state . end () } }
};
}
