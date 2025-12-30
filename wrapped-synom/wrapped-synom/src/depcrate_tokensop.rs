// Generated macro for op (macro)
macro_rules! Depcrate_tokensop {
() => {
// Module: crate::tokens
// Provides: {"op"}
// Dependencies: {}
macro_rules ! op { (pub struct $ name : ident ($ ($ contents : tt) *) => $ s : expr) => { # [cfg_attr (feature = "clone-impls" , derive (Copy , Clone))] # [cfg_attr (feature = "extra-traits" , derive (Debug , Eq , PartialEq , Hash))] # [derive (Default)] pub struct $ name (pub $ ($ contents) *) ; # [cfg (feature = "printing")] impl :: quote :: ToTokens for $ name { fn to_tokens (& self , tokens : & mut :: quote :: Tokens) { printing :: op ($ s , & self . 0 , tokens) ; } } # [cfg (feature = "parsing")] impl :: Synom for $ name { fn parse (tokens : $ crate :: Cursor) -> $ crate :: PResult <$ name > { parsing :: op ($ s , tokens , $ name) } } } }
};
}
