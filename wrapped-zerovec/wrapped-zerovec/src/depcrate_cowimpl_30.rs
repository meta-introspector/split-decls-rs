// Generated macro for impl_30 (impl)
macro_rules! Depcrate_cowimpl_30 {
() => {
// Module: crate::cow
// Provides: {"impl_30"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'a , V : VarULE + ? Sized + serde :: Serialize > serde :: Serialize for VarZeroCow < 'a , V > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { if serializer . is_human_readable () { < V as serde :: Serialize > :: serialize (self . deref () , serializer) } else { serializer . serialize_bytes (self . as_bytes ()) } } }
};
}
