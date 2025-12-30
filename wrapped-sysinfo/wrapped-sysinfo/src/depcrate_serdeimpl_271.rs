// Generated macro for impl_271 (impl)
macro_rules! Depcrate_serdeimpl_271 {
() => {
// Module: crate::serde
// Provides: {"impl_271"}
// Dependencies: {}
# [cfg (feature = "component")] impl Serialize for crate :: Component { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_struct ("Component" , 4) ? ; state . serialize_field ("temperature" , & self . temperature ()) ? ; state . serialize_field ("max" , & self . max ()) ? ; state . serialize_field ("critical" , & self . critical ()) ? ; state . serialize_field ("label" , & self . label ()) ? ; state . end () } }
};
}
