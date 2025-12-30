// Generated macro for impl_206 (impl)
macro_rules! Depcrate_tinyvecimpl_206 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_206"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , A : Array > Visitor < 'de > for TinyVecVisitor < A > where A :: Item : Deserialize < 'de > , { type Value = TinyVec < A > ; fn expecting (& self , formatter : & mut core :: fmt :: Formatter ,) -> core :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < S > (self , mut seq : S) -> Result < Self :: Value , S :: Error > where S : SeqAccess < 'de > , { let mut new_tinyvec = match seq . size_hint () { Some (expected_size) => TinyVec :: with_capacity (expected_size) , None => Default :: default () , } ; while let Some (value) = seq . next_element () ? { new_tinyvec . push (value) ; } Ok (new_tinyvec) } }
};
}
