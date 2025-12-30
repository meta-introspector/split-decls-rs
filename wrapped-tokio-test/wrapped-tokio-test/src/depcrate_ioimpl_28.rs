// Generated macro for impl_28 (impl)
macro_rules! Depcrate_ioimpl_28 {
() => {
// Module: crate::io
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'a > fmt :: Display for PanicMsgSnippet < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . 0 . name . is_empty () { write ! (f , "({} actions remain)" , self . 0 . actions . len ()) } else { write ! (f , "(name {}, {} actions remain)" , self . 0 . name , self . 0 . actions . len ()) } } }
};
}
