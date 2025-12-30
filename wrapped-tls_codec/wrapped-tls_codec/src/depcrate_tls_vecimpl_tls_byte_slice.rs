// Generated macro for impl_tls_byte_slice (macro)
macro_rules! Depcrate_tls_vecimpl_tls_byte_slice {
() => {
// Module: crate::tls_vec
// Provides: {"impl_tls_byte_slice"}
// Dependencies: {}
macro_rules ! impl_tls_byte_slice { ($ size : ty , $ name : ident , $ len_len : literal) => { pub struct $ name <'a > (pub &'a [u8]) ; impl <'a > $ name <'a > { # [doc = " Get the raw slice."] # [inline (always)] pub fn as_slice (& self) -> & [u8] { self . 0 } } impl <'a > $ name <'a > { impl_serialize_common ! (self , $ size , $ name , $ len_len , # [cfg (feature = "std")]) ; impl_byte_serialize ! (self , $ size , $ name , $ len_len) ; impl_byte_size ! (self , $ size , $ name , $ len_len) ; } impl <'a > Serialize for &$ name <'a > { # [cfg (feature = "std")] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { self . serialize_bytes (writer) } } impl <'a > Serialize for $ name <'a > { # [cfg (feature = "std")] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { self . serialize_bytes (writer) } } impl <'a > Size for &$ name <'a > { # [inline] fn tls_serialized_len (& self) -> usize { self . tls_serialized_byte_length () } } impl <'a > Size for $ name <'a > { # [inline] fn tls_serialized_len (& self) -> usize { self . tls_serialized_byte_length () } } } ; }
};
}
