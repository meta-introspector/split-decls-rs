// Generated macro for impl_148 (impl)
macro_rules! Depcrate_errorimpl_148 {
() => {
// Module: crate::error
// Provides: {"impl_148"}
// Dependencies: {}
impl Error { # [doc = " Create a new parse error from the given message."] pub (crate) fn parse (msg : String) -> Error { Error { kind : ErrorKind :: Parse (msg) , line : None , path : None } } # [doc = " Return the specific kind of this error."] pub fn kind (& self) -> & ErrorKind { & self . kind } # [doc = " Return the line number at which this error occurred, if available."] pub fn line (& self) -> Option < u64 > { self . line } # [doc = " Return the file path associated with this error, if one exists."] pub fn path (& self) -> Option < & Path > { self . path . as_ref () . map (| p | & * * p) } # [doc = " Unwrap this error into its underlying kind."] pub fn into_kind (self) -> ErrorKind { self . kind } # [doc = " Returns true if and only if this is an I/O error."] # [doc = ""] # [doc = " If this returns true, the underlying `ErrorKind` is guaranteed to be"] # [doc = " `ErrorKind::Io`."] pub fn is_io_error (& self) -> bool { match self . kind { ErrorKind :: Io (_) => true , _ => false , } } }
};
}
