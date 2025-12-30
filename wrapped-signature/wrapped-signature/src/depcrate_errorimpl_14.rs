// Generated macro for impl_14 (impl)
macro_rules! Depcrate_errorimpl_14 {
() => {
// Module: crate::error
// Provides: {"impl_14"}
// Dependencies: {}
impl Debug for Error { # [cfg (not (feature = "alloc"))] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("signature::Error {}") } # [cfg (feature = "alloc")] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("signature::Error { source: ") ? ; if let Some (source) = & self . source { write ! (f , "Some({source})") ? ; } else { f . write_str ("None") ? ; } f . write_str (" }") } }
};
}
