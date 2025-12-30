// Generated macro for impl_90 (impl)
macro_rules! Depcrate_deimpl_90 {
() => {
// Module: crate::de
// Provides: {"impl_90"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'de : 'a , 'a > Deserialize < 'de > for Cow < 'a , [u8] > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct CowVisitor ; impl < 'de > Visitor < 'de > for CowVisitor { type Value = Cow < 'de , [u8] > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a byte array") } fn visit_borrowed_bytes < E > (self , v : & 'de [u8]) -> Result < Self :: Value , E > where E : Error , { Ok (Cow :: Borrowed (v)) } fn visit_borrowed_str < E > (self , v : & 'de str) -> Result < Self :: Value , E > where E : Error , { Ok (Cow :: Borrowed (v . as_bytes ())) } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : Error , { Ok (Cow :: Owned (v . to_vec ())) } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : Error , { Ok (Cow :: Owned (v . as_bytes () . to_vec ())) } fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : Error , { Ok (Cow :: Owned (v)) } fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > where E : Error , { Ok (Cow :: Owned (v . into_bytes ())) } fn visit_seq < V > (self , mut visitor : V) -> Result < Self :: Value , V :: Error > where V : SeqAccess < 'de > , { let len = cmp :: min (visitor . size_hint () . unwrap_or (0) , 4096) ; let mut bytes = Vec :: with_capacity (len) ; while let Some (b) = visitor . next_element () ? { bytes . push (b) ; } Ok (Cow :: Owned (bytes)) } } deserializer . deserialize_bytes (CowVisitor) } }
};
}
