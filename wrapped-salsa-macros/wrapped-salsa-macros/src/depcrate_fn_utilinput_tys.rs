// Generated macro for input_tys (function)
macro_rules! Depcrate_fn_utilinput_tys {
() => {
// Module: crate::fn_util
// Provides: {"input_tys"}
// Dependencies: {}
pub fn input_tys (sig : & syn :: Signature , skip : usize) -> syn :: Result < Vec < & syn :: Type > > { sig . inputs . iter () . skip (skip) . map (| input | { if let syn :: FnArg :: Typed (typed) = input { Ok (& * typed . ty) } else { Err (syn :: Error :: new_spanned (input , "unexpected receiver")) } }) . collect () }
};
}
