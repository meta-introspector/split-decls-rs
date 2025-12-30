// Generated macro for option_ty (function)
macro_rules! Depcrate_utiloption_ty {
() => {
// Module: crate::util
// Provides: {"option_ty"}
// Dependencies: {}
# [doc = " From `T` create `Option<T>`"] pub (crate) fn option_ty (t : syn :: Type) -> syn :: Type { let arguments = syn :: PathArguments :: AngleBracketed (syn :: AngleBracketedGenericArguments { colon2_token : None , lt_token : Default :: default () , args : FromIterator :: from_iter (vec ! [syn :: GenericArgument :: Type (t)]) , gt_token : Default :: default () , }) ; let ident = raw_ident ("Option") ; let seg = syn :: PathSegment { ident , arguments } ; let path : syn :: Path = seg . into () ; let ty = syn :: TypePath { qself : None , path } ; ty . into () }
};
}
