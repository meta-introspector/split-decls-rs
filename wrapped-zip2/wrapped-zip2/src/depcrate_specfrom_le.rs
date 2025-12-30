// Generated macro for from_le (macro)
macro_rules! Depcrate_specfrom_le {
() => {
// Module: crate::spec
// Provides: {"from_le"}
// Dependencies: {}
# [doc = " Convert all the fields of a struct *from* little-endian representations."] macro_rules ! from_le { ($ obj : ident , $ field : ident , $ type : ty) => { $ obj .$ field = <$ type >:: from_le ($ obj .$ field) ; } ; ($ obj : ident , [($ field : ident , $ type : ty) $ (,) ?]) => { from_le ! [$ obj , $ field , $ type] ; } ; ($ obj : ident , [($ field : ident , $ type : ty) , $ ($ rest : tt) ,+ $ (,) ?]) => { from_le ! [$ obj , $ field , $ type] ; from_le ! ($ obj , [$ ($ rest) ,+]) ; } ; }
};
}
