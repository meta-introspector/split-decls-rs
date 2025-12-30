// Generated macro for impl_209 (impl)
macro_rules! Depcrate_serdeimpl_209 {
() => {
// Module: crate::serde
// Provides: {"impl_209"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for ByteStrVisitor { type Value = Box < [u8] > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "a slice of borrowed bytes or a string") } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > { Ok (Box :: from (v)) } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > { Ok (Box :: from (v . as_bytes ())) } fn visit_seq < A > (self , mut v : A) -> Result < Self :: Value , A :: Error > where A : serde_core :: de :: SeqAccess < 'de > , { let mut result = Vec :: with_capacity (v . size_hint () . unwrap_or (0)) ; while let Some (x) = v . next_element :: < u8 > () ? { result . push (x) ; } Ok (Box :: from (result)) } }
};
}
