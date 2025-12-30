// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl StdError for Error { fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (match & self . kind { ErrorKind :: Io { inner , .. } => inner , ErrorKind :: Os (err) => & * * err , ErrorKind :: Pem (err) => err , }) } }
};
}
