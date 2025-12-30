// Generated macro for impl_tls_slice (macro)
macro_rules! Depcrate_tls_vecimpl_tls_slice {
() => {
// Module: crate::tls_vec
// Provides: {"impl_tls_slice"}
// Dependencies: {}
macro_rules ! impl_tls_slice { ($ size : ty , $ name : ident , $ len_len : literal) => { pub struct $ name <'a , T > (pub &'a [T]) ; impl <'a , T > $ name <'a , T > { # [doc = " Get the raw slice."] # [inline (always)] pub fn as_slice (& self) -> & [T] { self . 0 } } impl <'a , T : Size > $ name <'a , T > { impl_size ! (self , $ size , $ name , $ len_len) ; } impl <'a , T : Serialize > $ name <'a , T > { impl_serialize_common ! (self , $ size , $ name , $ len_len , # [cfg (feature = "std")]) ; impl_serialize ! (self , $ size , $ name , $ len_len) ; } impl <'a , T : Serialize > Serialize for &$ name <'a , T > { # [cfg (feature = "std")] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { self . serialize (writer) } } impl <'a , T : Serialize > Serialize for $ name <'a , T > { # [cfg (feature = "std")] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { self . serialize (writer) } } impl <'a , T : Size > Size for &$ name <'a , T > { # [inline] fn tls_serialized_len (& self) -> usize { self . tls_serialized_length () } } impl <'a , T : Size > Size for $ name <'a , T > { # [inline] fn tls_serialized_len (& self) -> usize { self . tls_serialized_length () } } } ; }
};
}
