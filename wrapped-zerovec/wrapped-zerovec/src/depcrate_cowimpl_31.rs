// Generated macro for impl_31 (impl)
macro_rules! Depcrate_cowimpl_31 {
() => {
// Module: crate::cow
// Provides: {"impl_31"}
// Dependencies: {}
# [cfg (all (feature = "serde" , feature = "alloc"))] impl < 'a , 'de : 'a , V : VarULE + ? Sized > serde :: Deserialize < 'de > for VarZeroCow < 'a , V > where Box < V > : serde :: Deserialize < 'de > , { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : serde :: Deserializer < 'de > , { if deserializer . is_human_readable () { let b = Box :: < V > :: deserialize (deserializer) ? ; Ok (Self :: new_owned (b)) } else { let bytes = < & [u8] > :: deserialize (deserializer) ? ; Self :: parse_bytes (bytes) . map_err (serde :: de :: Error :: custom) } } }
};
}
