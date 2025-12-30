// Generated macro for to_le (macro)
macro_rules! Depcrate_specto_le {
() => {
// Module: crate::spec
// Provides: {"to_le"}
// Dependencies: {}
# [doc = " Convert all the fields of a struct *into* little-endian representations."] macro_rules ! to_le { ($ obj : ident , $ field : ident , $ type : ty) => { $ obj .$ field = <$ type >:: to_le ($ obj .$ field) ; } ; ($ obj : ident , [($ field : ident , $ type : ty) $ (,) ?]) => { to_le ! [$ obj , $ field , $ type] ; } ; ($ obj : ident , [($ field : ident , $ type : ty) , $ ($ rest : tt) ,+ $ (,) ?]) => { to_le ! [$ obj , $ field , $ type] ; to_le ! ($ obj , [$ ($ rest) ,+]) ; } ; }
};
}
