// Generated macro for impl_23 (impl)
macro_rules! Depcrate_de_valueimpl_23 {
() => {
// Module: crate::de::value
// Provides: {"impl_23"}
// Dependencies: {}
impl Display for Error { # [cfg (any (feature = "std" , feature = "alloc"))] fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (& self . err) } # [cfg (not (any (feature = "std" , feature = "alloc")))] fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("Serde deserialization error") } }
};
}
