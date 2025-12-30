// Generated macro for impl_fmt_traits (macro)
macro_rules! Depcrate_fmtimpl_fmt_traits {
() => {
// Module: crate::fmt
// Provides: {"impl_fmt_traits"}
// Dependencies: {}
macro_rules ! impl_fmt_traits { ($ ($ T : ident <$ ($ a : lifetime) ,*>) ,+) => { $ (impl <$ ($ a) ,*> fmt :: Display for $ T <$ ($ a) ,*> { # [inline] fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (self , f) } } impl <$ ($ a) ,*> fmt :: LowerHex for $ T <$ ($ a) ,*> { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { f . write_str (self . encode_lower (& mut [0 ; Self :: LENGTH])) } } impl <$ ($ a) ,*> fmt :: UpperHex for $ T <$ ($ a) ,*> { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { f . write_str (self . encode_upper (& mut [0 ; Self :: LENGTH])) } } impl_fmt_from ! ($ T <$ ($ a) ,*>) ;) + } }
};
}
