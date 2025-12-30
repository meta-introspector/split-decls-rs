// Generated macro for vec_ty (function)
macro_rules! Depcrate_utilvec_ty {
() => {
// Module: crate::util
// Provides: {"vec_ty"}
// Dependencies: {}
# [doc = " From `T` create `alloc::Vec<T>`."] pub (crate) fn vec_ty (t : syn :: Type) -> syn :: Type { let arguments = syn :: PathArguments :: AngleBracketed (syn :: AngleBracketedGenericArguments { colon2_token : None , lt_token : Default :: default () , args : FromIterator :: from_iter (vec ! [syn :: GenericArgument :: Type (t)]) , gt_token : Default :: default () , }) ; let mut path = syn :: Path { leading_colon : Some (Default :: default ()) , segments : Punctuated :: new () , } ; path . segments . push (raw_ident ("alloc") . into ()) ; path . segments . push (raw_ident ("vec") . into ()) ; path . segments . push (syn :: PathSegment { ident : raw_ident ("Vec") , arguments , }) ; let ty = syn :: TypePath { qself : None , path } ; ty . into () }
};
}
