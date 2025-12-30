// Generated macro for impl_763 (impl)
macro_rules! Depcrate_ule_vartupleimpl_763 {
() => {
// Module: crate::ule::vartuple
// Provides: {"impl_763"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'a , 'de : 'a , A , V > serde :: Deserialize < 'de > for & 'a VarTupleULE < A , V > where A : AsULE + 'static , V : VarULE + ? Sized , A : serde :: Deserialize < 'de > , { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : serde :: Deserializer < 'de > , { if ! deserializer . is_human_readable () { let bytes = < & [u8] > :: deserialize (deserializer) ? ; VarTupleULE :: < A , V > :: parse_bytes (bytes) . map_err (serde :: de :: Error :: custom) } else { Err (serde :: de :: Error :: custom ("&VarTupleULE can only deserialize in zero-copy ways" ,)) } } }
};
}
