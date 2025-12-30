// Generated macro for impl_668 (impl)
macro_rules! Depcrate_ty_kindimpl_668 {
() => {
// Module: crate::ty_kind
// Provides: {"impl_668"}
// Dependencies: {}
impl < I : Interner > fmt :: Debug for FnSig < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let sig = self ; let FnSig { inputs_and_output : _ , c_variadic , safety , abi } = sig ; write ! (f , "{}" , safety . prefix_str ()) ? ; if ! abi . is_rust () { write ! (f , "extern \"{abi:?}\" ") ? ; } write ! (f , "fn(") ? ; let inputs = sig . inputs () ; for (i , ty) in inputs . iter () . enumerate () { if i > 0 { write ! (f , ", ") ? ; } write ! (f , "{ty:?}") ? ; } if * c_variadic { if inputs . is_empty () { write ! (f , "...") ? ; } else { write ! (f , ", ...") ? ; } } write ! (f , ")") ? ; let output = sig . output () ; match output . kind () { Tuple (list) if list . is_empty () => Ok (()) , _ => write ! (f , " -> {:?}" , sig . output ()) , } } }
};
}
