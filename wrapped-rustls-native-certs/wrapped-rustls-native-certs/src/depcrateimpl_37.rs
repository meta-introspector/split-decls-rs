// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self . context) ? ; f . write_str (": ") ? ; match & self . kind { ErrorKind :: Io { inner , path } => { write ! (f , "{inner} at '{}'" , path . display ()) } ErrorKind :: Os (err) => err . fmt (f) , ErrorKind :: Pem (err) => err . fmt (f) , } } }
};
}
