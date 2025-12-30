// Generated macro for impl_40 (impl)
macro_rules! Depcrate_internals_attrimpl_40 {
() => {
// Module: crate::internals::attr
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'c , T > VecAttr < 'c , T > { fn none (cx : & 'c Ctxt , name : Symbol) -> Self { VecAttr { cx , name , first_dup_tokens : TokenStream :: new () , values : Vec :: new () , } } fn insert < A : ToTokens > (& mut self , obj : A , value : T) { if self . values . len () == 1 { self . first_dup_tokens = obj . into_token_stream () ; } self . values . push (value) ; } fn at_most_one (mut self) -> Option < T > { if self . values . len () > 1 { let dup_token = self . first_dup_tokens ; let msg = format ! ("duplicate serde attribute `{}`" , self . name) ; self . cx . error_spanned_by (dup_token , msg) ; None } else { self . values . pop () } } pub (crate) fn get (self) -> Vec < T > { self . values } }
};
}
