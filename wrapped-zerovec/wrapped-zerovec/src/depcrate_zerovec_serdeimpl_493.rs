// Generated macro for impl_493 (impl)
macro_rules! Depcrate_zerovec_serdeimpl_493 {
() => {
// Module: crate::zerovec::serde
// Provides: {"impl_493"}
// Dependencies: {}
impl < 'de , T > Visitor < 'de > for ZeroVecVisitor < T > where T : 'de + Deserialize < 'de > + AsULE , { type Value = ZeroVec < 'de , T > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a sequence or borrowed buffer of fixed-width elements") } fn visit_borrowed_bytes < E > (self , bytes : & 'de [u8]) -> Result < Self :: Value , E > where E : de :: Error , { ZeroVec :: parse_bytes (bytes) . map_err (de :: Error :: custom) } # [cfg (feature = "alloc")] fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut vec : alloc :: vec :: Vec < T :: ULE > = if let Some (capacity) = seq . size_hint () { alloc :: vec :: Vec :: with_capacity (capacity) } else { alloc :: vec :: Vec :: new () } ; while let Some (value) = seq . next_element :: < T > () ? { vec . push (T :: to_unaligned (value)) ; } Ok (ZeroVec :: new_owned (vec)) } }
};
}
