// Generated macro for delim (macro)
macro_rules! Depcrate_tokensdelim {
() => {
// Module: crate::tokens
// Provides: {"delim"}
// Dependencies: {}
macro_rules ! delim { (pub struct $ name : ident => $ s : expr) => { # [cfg_attr (feature = "clone-impls" , derive (Copy , Clone))] # [cfg_attr (feature = "extra-traits" , derive (Debug , Eq , PartialEq , Hash))] # [derive (Default)] pub struct $ name (pub Span) ; impl $ name { # [cfg (feature = "printing")] pub fn surround < F > (& self , tokens : & mut :: quote :: Tokens , f : F) where F : FnOnce (& mut :: quote :: Tokens) { printing :: delim ($ s , & self . 0 , tokens , f) ; } # [cfg (feature = "parsing")] pub fn parse < F , R > (tokens : $ crate :: Cursor , f : F) -> $ crate :: PResult < (R , $ name) > where F : FnOnce ($ crate :: Cursor) -> $ crate :: PResult < R > { parsing :: delim ($ s , tokens , $ name , f) } } } }
};
}
