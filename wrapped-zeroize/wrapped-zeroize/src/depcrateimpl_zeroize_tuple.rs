// Generated macro for impl_zeroize_tuple (macro)
macro_rules! Depcrateimpl_zeroize_tuple {
() => {
// Module: crate
// Provides: {"impl_zeroize_tuple"}
// Dependencies: {}
macro_rules ! impl_zeroize_tuple { ($ ($ type_name : ident) ,+) => { impl <$ ($ type_name : Zeroize) ,+> Zeroize for ($ ($ type_name ,) +) { fn zeroize (& mut self) { # [allow (non_snake_case)] let ($ ($ type_name ,) +) = self ; $ ($ type_name . zeroize ()) ;+ } } impl <$ ($ type_name : ZeroizeOnDrop) ,+> ZeroizeOnDrop for ($ ($ type_name ,) +) { } } }
};
}
