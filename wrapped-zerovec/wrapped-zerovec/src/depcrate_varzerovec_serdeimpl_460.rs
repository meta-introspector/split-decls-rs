// Generated macro for impl_460 (impl)
macro_rules! Depcrate_varzerovec_serdeimpl_460 {
() => {
// Module: crate::varzerovec::serde
// Provides: {"impl_460"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'de , T , F > Visitor < 'de > for VarZeroVecHumanVisitor < T , F > where T : VarULE + ? Sized , Box < T > : Deserialize < 'de > , F : VarZeroVecFormat , { type Value = VarZeroVec < 'de , T , F > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a sequence or borrowed buffer of bytes") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut vec : Vec < Box < T > > = if let Some (capacity) = seq . size_hint () { Vec :: with_capacity (capacity) } else { Vec :: new () } ; while let Some (value) = seq . next_element :: < Box < T > > () ? { vec . push (value) ; } Ok (VarZeroVec :: from (& vec)) } }
};
}
