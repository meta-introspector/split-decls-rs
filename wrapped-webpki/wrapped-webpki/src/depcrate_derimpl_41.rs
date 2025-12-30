// Generated macro for impl_41 (impl)
macro_rules! Depcrate_derimpl_41 {
() => {
// Module: crate::der
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'a > FromDer < 'a > for u8 { fn from_der (reader : & mut untrusted :: Reader < 'a >) -> Result < Self , Error > { match * nonnegative_integer (reader) ? . as_slice_less_safe () { [b] => Ok (b) , _ => Err (Error :: BadDer) , } } const TYPE_ID : DerTypeId = DerTypeId :: U8 ; }
};
}
