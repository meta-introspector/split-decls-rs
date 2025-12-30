// Generated macro for impl_15 (impl)
macro_rules! Depcrate_errorimpl_15 {
() => {
// Module: crate::error
// Provides: {"impl_15"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & * self . kind { ErrorKind :: CurrentDir { err , path } => { let suffix = path . as_ref () . map_or (String :: new () , | path | format ! (" `{}`" , path . display ())) ; write ! (f , "failed to get current directory{suffix}: {err}") } ErrorKind :: Var { err , var } => { let var = var . to_string_lossy () ; write ! (f , "failed to get environment variable `{var}`: {err}") } ErrorKind :: ReadFile { err , path } => { let path = path . display () ; write ! (f , "failed to read file `{path}`: {err}") } ErrorKind :: ReadDir { err , path } => { let path = path . display () ; write ! (f , "failed read directory `{path}`: {err}") } ErrorKind :: WriteFile { err , path } => { let path = path . display () ; write ! (f , "failed to write file `{path}`: {err}") } ErrorKind :: CopyFile { err , src , dst } => { let src = src . display () ; let dst = dst . display () ; write ! (f , "failed to copy `{src}` to `{dst}`: {err}") } ErrorKind :: HardLink { err , src , dst } => { let src = src . display () ; let dst = dst . display () ; write ! (f , "failed hard link `{src}` to `{dst}`: {err}") } ErrorKind :: CreateDir { err , path } => { let path = path . display () ; write ! (f , "failed to create directory `{path}`: {err}") } ErrorKind :: RemovePath { err , path } => { let path = path . display () ; write ! (f , "failed to remove path `{path}`: {err}") } ErrorKind :: Cmd (cmd) => fmt :: Display :: fmt (cmd , f) , } ? ; Ok (()) } }
};
}
