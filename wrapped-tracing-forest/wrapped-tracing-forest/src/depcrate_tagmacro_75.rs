// Generated macro for macro_75 (macro)
macro_rules! Depcrate_tagmacro_75 {
() => {
// Module: crate::tag
// Provides: {"macro_75"}
// Dependencies: {}
cfg_serde ! { use serde :: { Serialize , Serializer } ; impl Serialize for Tag { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { serializer . serialize_str (& self . to_string ()) } } }
};
}
