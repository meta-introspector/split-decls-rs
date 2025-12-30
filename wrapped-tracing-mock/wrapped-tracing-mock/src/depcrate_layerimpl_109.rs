// Generated macro for impl_109 (impl)
macro_rules! Depcrate_layerimpl_109 {
() => {
// Module: crate::layer
// Provides: {"impl_109"}
// Dependencies: {}
impl fmt :: Debug for MockLayer { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("ExpectSubscriber") ; s . field ("name" , & self . name) ; if let Ok (expected) = self . expected . try_lock () { s . field ("expected" , & expected) ; } else { s . field ("expected" , & format_args ! ("<locked>")) ; } if let Ok (current) = self . current . try_lock () { s . field ("current" , & format_args ! ("{:?}" , & current)) ; } else { s . field ("current" , & format_args ! ("<locked>")) ; } s . finish () } }
};
}
