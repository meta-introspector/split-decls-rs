// Generated macro for impl_150 (impl)
macro_rules! Depcrate_errorimpl_150 {
() => {
// Module: crate::error
// Provides: {"impl_150"}
// Dependencies: {}
impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if let Some (ref path) = self . path { if let Some (line) = self . line { write ! (f , "{}:{}: " , path . display () , line) ? ; } else { write ! (f , "{}: " , path . display ()) ? ; } } else if let Some (line) = self . line { write ! (f , "error on line {}: " , line) ? ; } match self . kind { ErrorKind :: Io (ref err) => write ! (f , "{}" , err) , ErrorKind :: Parse (ref msg) => write ! (f , "{}" , msg) , } } }
};
}
