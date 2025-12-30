// Generated macro for impl_for_tuple (macro)
macro_rules! Depcrate_web_messageimpl_for_tuple {
() => {
// Module: crate::web::message
// Provides: {"impl_for_tuple"}
// Dependencies: {}
# [doc = " Implement a trait for a tuple."] macro_rules ! impl_for_tuple { ($ _0 : literal , $ trait : ident , $ ($ generic : ident => $ _1 : tt) ,+) => { impl <$ ($ generic) ,+> $ trait for ($ ($ generic ,) +) { } } ; }
};
}
