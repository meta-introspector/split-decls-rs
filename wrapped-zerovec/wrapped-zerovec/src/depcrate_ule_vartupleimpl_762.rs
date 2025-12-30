// Generated macro for impl_762 (impl)
macro_rules! Depcrate_ule_vartupleimpl_762 {
() => {
// Module: crate::ule::vartuple
// Provides: {"impl_762"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < A , V > serde :: Serialize for VarTupleULE < A , V > where A : AsULE + 'static , V : VarULE + ? Sized , A : serde :: Serialize , V : serde :: Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { if serializer . is_human_readable () { let this = VarTuple { sized : A :: from_unaligned (self . sized) , variable : & self . variable , } ; this . serialize (serializer) } else { serializer . serialize_bytes (self . as_bytes ()) } } }
};
}
