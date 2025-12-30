// Generated macro for serde_impl (module)
macro_rules! Depcrate_quic_vecserde_impl {
() => {
// Module: crate::quic_vec
// Provides: {"serde_impl"}
// Dependencies: {}
# [cfg (feature = "serde")] mod serde_impl { use std :: { fmt , vec :: Vec } ; use serde :: { Deserializer , de } ; pub (super) fn de_vec_bytes_compat < 'de , D > (deserializer : D) -> Result < Vec < u8 > , D :: Error > where D : Deserializer < 'de > , { struct BytesOrSeq ; impl < 'de > de :: Visitor < 'de > for BytesOrSeq { type Value = Vec < u8 > ; fn expecting (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("either a byte blob or a sequence of u8") } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : de :: Error , { Ok (v . to_vec ()) } fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : de :: Error , { Ok (v) } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : de :: SeqAccess < 'de > , { let mut out = Vec :: new () ; while let Some (b) = seq . next_element :: < u8 > () ? { out . push (b) ; } Ok (out) } } deserializer . deserialize_any (BytesOrSeq) } }
};
}
