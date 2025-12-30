// Generated macro for impl_trait_result (function)
macro_rules! Depcrate_syn_utilsimpl_trait_result {
() => {
// Module: crate::syn_utils
// Provides: {"impl_trait_result"}
// Dependencies: {}
pub fn impl_trait_result (input : & DeriveInput , trait_path : & Path , wheres : & [WherePredicate] , contents : TokenStream , dump : bool ,) -> Result < TokenStream > { let ts = impl_trait (input , trait_path , wheres , contents) ; if dump { panic ! ("macro result: \n{ts}") ; } Ok (ts) }
};
}
