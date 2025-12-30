// Generated macro for impl_117 (impl)
macro_rules! Depcrate_commonimpl_117 {
() => {
// Module: crate::common
// Provides: {"impl_117"}
// Dependencies: {}
impl < D > UcdLineParser < File , D > { # [doc = " Create a new parser from the given file path."] pub (crate) fn from_path < P : AsRef < Path > > (path : P ,) -> Result < UcdLineParser < File , D > , Error > { let path = path . as_ref () ; let file = File :: open (path) . map_err (| e | Error { kind : ErrorKind :: Io (e) , line : None , path : Some (path . to_path_buf ()) , }) ? ; Ok (UcdLineParser :: new (Some (path . to_path_buf ()) , file)) } }
};
}
