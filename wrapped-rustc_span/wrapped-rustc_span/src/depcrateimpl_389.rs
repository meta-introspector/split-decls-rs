// Generated macro for impl_389 (impl)
macro_rules! Depcrateimpl_389 {
() => {
// Module: crate
// Provides: {"impl_389"}
// Dependencies: {}
impl Display for SourceFileHash { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}=" , self . kind) ? ; for byte in self . value [0 .. self . hash_len ()] . into_iter () { write ! (f , "{byte:02x}") ? ; } Ok (()) } }
};
}
