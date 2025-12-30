// Generated macro for impl_764 (impl)
macro_rules! Depcrate_ule_vartupleimpl_764 {
() => {
// Module: crate::ule::vartuple
// Provides: {"impl_764"}
// Dependencies: {}
# [cfg (all (feature = "serde" , feature = "alloc"))] impl < 'de , A , V > serde :: Deserialize < 'de > for alloc :: boxed :: Box < VarTupleULE < A , V > > where A : AsULE + 'static , V : VarULE + ? Sized , A : serde :: Deserialize < 'de > , alloc :: boxed :: Box < V > : serde :: Deserialize < 'de > , { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : serde :: Deserializer < 'de > , { if deserializer . is_human_readable () { let this = VarTuple :: < A , alloc :: boxed :: Box < V > > :: deserialize (deserializer) ? ; Ok (crate :: ule :: encode_varule_to_box (& this)) } else { let deserialized = < & VarTupleULE < A , V > > :: deserialize (deserializer) ? ; Ok (deserialized . to_boxed ()) } } }
};
}
