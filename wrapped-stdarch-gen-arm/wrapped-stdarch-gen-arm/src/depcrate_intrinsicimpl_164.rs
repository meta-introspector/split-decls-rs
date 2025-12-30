// Generated macro for impl_164 (impl)
macro_rules! Depcrate_intrinsicimpl_164 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_164"}
// Dependencies: {}
impl ToTokens for Intrinsic { fn to_tokens (& self , tokens : & mut TokenStream) { if ! self . big_endian_compose . is_empty () { for i in 0 .. 2 { match i { 0 => create_tokens (self , Endianness :: Little , tokens) , 1 => create_tokens (self , Endianness :: Big , tokens) , _ => panic ! ("Currently only little and big endian exist") , } } } else { create_tokens (self , Endianness :: NA , tokens) ; } } }
};
}
