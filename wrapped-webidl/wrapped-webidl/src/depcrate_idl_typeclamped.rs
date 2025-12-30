// Generated macro for clamped (function)
macro_rules! Depcrate_idl_typeclamped {
() => {
// Module: crate::idl_type
// Provides: {"clamped"}
// Dependencies: {}
# [doc = " From `T` create `::wasm_bindgen::Clamped<T>`"] fn clamped (t : syn :: Type) -> syn :: Type { let arguments = syn :: PathArguments :: AngleBracketed (syn :: AngleBracketedGenericArguments { colon2_token : None , lt_token : Default :: default () , args : vec ! [syn :: GenericArgument :: Type (t)] . into_iter () . collect () , gt_token : Default :: default () , }) ; let ident = raw_ident ("Clamped") ; let seg = syn :: PathSegment { ident , arguments } ; syn :: TypePath { qself : None , path : syn :: Path { leading_colon : Some (Default :: default ()) , segments : vec ! [Ident :: new ("wasm_bindgen" , Span :: call_site ()) . into () , seg] . into_iter () . collect () , } , } . into () }
};
}
