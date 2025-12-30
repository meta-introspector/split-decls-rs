// Generated macro for impl_24 (impl)
macro_rules! Depcrate_de_valueimpl_24 {
() => {
// Module: crate::de::value
// Provides: {"impl_24"}
// Dependencies: {}
impl Debug for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let mut debug = formatter . debug_tuple ("Error") ; # [cfg (any (feature = "std" , feature = "alloc"))] debug . field (& self . err) ; debug . finish () } }
};
}
