// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl fmt :: Display for Digest { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { for i in self . data . state . iter () { r#try ! (write ! (f , "{:08x}" , i)) ; } Ok (()) } }
};
}
