// Generated macro for to_and_from_le (macro)
macro_rules! Depcrate_specto_and_from_le {
() => {
// Module: crate::spec
// Provides: {"to_and_from_le"}
// Dependencies: {}
# [doc = " Implement `from_le()` and `to_le()`, providing the field specification to both macros"] # [doc = " and methods."] macro_rules ! to_and_from_le { ($ ($ args : tt) ,+ $ (,) ?) => { # [inline (always)] fn from_le (mut self) -> Self { from_le ! [self , [$ ($ args) ,+]] ; self } # [inline (always)] fn to_le (mut self) -> Self { to_le ! [self , [$ ($ args) ,+]] ; self } } ; }
};
}
