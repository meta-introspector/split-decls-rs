// Generated macro for ErrorKind (enum)
macro_rules! DepcrateErrorKind {
() => {
// Module: crate
// Provides: {"ErrorKind"}
// Dependencies: {}
# [non_exhaustive] # [derive (Debug)] pub enum ErrorKind { Io { inner : io :: Error , path : PathBuf } , Os (Box < dyn StdError + Send + Sync + 'static >) , Pem (pem :: Error) , }
};
}
