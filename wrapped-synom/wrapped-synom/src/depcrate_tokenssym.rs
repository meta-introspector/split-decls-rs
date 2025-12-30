// Generated macro for sym (macro)
macro_rules! Depcrate_tokenssym {
() => {
// Module: crate::tokens
// Provides: {"sym"}
// Dependencies: {}
macro_rules ! sym { (pub struct $ name : ident => $ s : expr) => { # [cfg_attr (feature = "clone-impls" , derive (Copy , Clone))] # [cfg_attr (feature = "extra-traits" , derive (Debug , Eq , PartialEq , Hash))] # [derive (Default)] pub struct $ name (pub Span) ; # [cfg (feature = "printing")] impl :: quote :: ToTokens for $ name { fn to_tokens (& self , tokens : & mut :: quote :: Tokens) { printing :: sym ($ s , & self . 0 , tokens) ; } } # [cfg (feature = "parsing")] impl :: Synom for $ name { fn parse (tokens : $ crate :: Cursor) -> $ crate :: PResult <$ name > { parsing :: sym ($ s , tokens , $ name) } } } }
};
}
