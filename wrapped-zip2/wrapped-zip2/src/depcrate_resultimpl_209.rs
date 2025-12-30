// Generated macro for impl_209 (impl)
macro_rules! Depcrate_resultimpl_209 {
() => {
// Module: crate::result
// Provides: {"impl_209"}
// Dependencies: {}
impl Display for ZipError { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { Self :: Io (_) => f . write_str ("i/o error") , Self :: InvalidArchive (e) => write ! (f , "invalid Zip archive: {e}") , Self :: UnsupportedArchive (e) => write ! (f , "unsupported Zip archive: {e}") , Self :: FileNotFound => f . write_str ("specified file not found in archive") , Self :: InvalidPassword => f . write_str ("provided password is incorrect") , } } }
};
}
