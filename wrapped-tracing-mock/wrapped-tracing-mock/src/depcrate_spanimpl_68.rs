// Generated macro for impl_68 (impl)
macro_rules! Depcrate_spanimpl_68 {
() => {
// Module: crate::span
// Provides: {"impl_68"}
// Dependencies: {}
impl fmt :: Debug for ExpectedSpan { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("MockSpan") ; if let Some (id) = self . id () { s . field ("id" , & id) ; } if let Some (name) = self . name () { s . field ("name" , & name) ; } if let Some (level) = self . level () { s . field ("level" , & format_args ! ("{:?}" , level)) ; } if let Some (target) = self . target () { s . field ("target" , & target) ; } s . finish () } }
};
}
