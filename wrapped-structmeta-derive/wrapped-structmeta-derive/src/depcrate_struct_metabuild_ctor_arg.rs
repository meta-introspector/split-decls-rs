// Generated macro for build_ctor_arg (function)
macro_rules! Depcrate_struct_metabuild_ctor_arg {
() => {
// Module: crate::struct_meta
// Provides: {"build_ctor_arg"}
// Dependencies: {}
fn build_ctor_arg (info : & ParamInfo , value : TokenStream , ctor_args : & mut [TokenStream]) { let value = if let Some (ident) = & info . field . ident { quote ! (# ident : # value) } else { value } ; ctor_args [info . index] = value ; }
};
}
