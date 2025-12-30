// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
# [cfg (feature = "std")] impl Serialize for SerializeError < '_ > { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { struct CollectStr < 'a > (& 'a dyn std :: error :: Error) ; impl Serialize for CollectStr < '_ > { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { serializer . collect_str (& self . 0) } } let mut s = serializer . serialize_struct ("Error" , 2) ? ; s . serialize_field ("message" , & CollectStr (self . 0)) ? ; s . serialize_field ("source" , & self . 0 . source () . map (SerializeError)) ? ; s . end () } }
};
}
