// Generated macro for impl_83 (impl)
macro_rules! Depcrate_errorimpl_83 {
() => {
// Module: crate::error
// Provides: {"impl_83"}
// Dependencies: {}
# [doc = " The Display implementation allows the `std::error::Error` implementation"] impl < I : Clone + fmt :: Display > fmt :: Display for InputError < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "failed to parse starting at: {}" , self . input) } }
};
}
