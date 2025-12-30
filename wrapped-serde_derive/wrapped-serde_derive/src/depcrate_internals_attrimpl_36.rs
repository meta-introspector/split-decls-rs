// Generated macro for impl_36 (impl)
macro_rules! Depcrate_internals_attrimpl_36 {
() => {
// Module: crate::internals::attr
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'c , T > Attr < 'c , T > { fn none (cx : & 'c Ctxt , name : Symbol) -> Self { Attr { cx , name , tokens : TokenStream :: new () , value : None , } } fn set < A : ToTokens > (& mut self , obj : A , value : T) { let tokens = obj . into_token_stream () ; if self . value . is_some () { let msg = format ! ("duplicate serde attribute `{}`" , self . name) ; self . cx . error_spanned_by (tokens , msg) ; } else { self . tokens = tokens ; self . value = Some (value) ; } } fn set_opt < A : ToTokens > (& mut self , obj : A , value : Option < T >) { if let Some (value) = value { self . set (obj , value) ; } } fn set_if_none (& mut self , value : T) { if self . value . is_none () { self . value = Some (value) ; } } pub (crate) fn get (self) -> Option < T > { self . value } fn get_with_tokens (self) -> Option < (TokenStream , T) > { match self . value { Some (v) => Some ((self . tokens , v)) , None => None , } } }
};
}
