// Generated macro for impl_98 (impl)
macro_rules! Depcrate_proptest_fnimpl_98 {
() => {
// Module: crate::proptest_fn
// Provides: {"impl_98"}
// Dependencies: {}
impl TestFnArg { fn from (arg : & FnArg) -> Result < Self > { if let FnArg :: Typed (arg) = arg { if let Pat :: Ident (ident) = arg . pat . as_ref () { if ident . attrs . is_empty () && ident . by_ref . is_none () && ident . subpat . is_none () { return Ok (Self { field : Field { attrs : arg . attrs . clone () , vis : Visibility :: Inherited , mutability : FieldMutability :: None , ident : Some (ident . ident . clone ()) , colon_token : Some (arg . colon_token) , ty : arg . ty . as_ref () . clone () , } , mutability : ident . mutability , }) ; } } else { bail ! (arg . pat . span () , "argument pattern not supported.") ; } } bail ! (arg . span () , "argument {} is not supported." , arg . to_token_stream ()) ; } fn pat (& self) -> TokenStream { let mutability = & self . mutability ; let ident = & self . field . ident ; quote ! (# mutability # ident) } }
};
}
