// Generated macro for impl_145 (impl)
macro_rules! Depcrate_bytebufimpl_145 {
() => {
// Module: crate::bytebuf
// Provides: {"impl_145"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for ByteBufVisitor { type Value = ByteBuf ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("byte array") } fn visit_seq < V > (self , mut visitor : V) -> Result < ByteBuf , V :: Error > where V : SeqAccess < 'de > , { let len = cmp :: min (visitor . size_hint () . unwrap_or (0) , 4096) ; let mut bytes = Vec :: with_capacity (len) ; while let Some (b) = visitor . next_element () ? { bytes . push (b) ; } Ok (ByteBuf :: from (bytes)) } fn visit_bytes < E > (self , v : & [u8]) -> Result < ByteBuf , E > where E : Error , { Ok (ByteBuf :: from (v)) } fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < ByteBuf , E > where E : Error , { Ok (ByteBuf :: from (v)) } fn visit_str < E > (self , v : & str) -> Result < ByteBuf , E > where E : Error , { Ok (ByteBuf :: from (v)) } fn visit_string < E > (self , v : String) -> Result < ByteBuf , E > where E : Error , { Ok (ByteBuf :: from (v)) } }
};
}
