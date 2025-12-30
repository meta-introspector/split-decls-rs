// Generated macro for impl_210 (impl)
macro_rules! Depcrate_resultimpl_210 {
() => {
// Module: crate::result
// Provides: {"impl_210"}
// Dependencies: {}
impl Error for ZipError { fn source (& self) -> Option < & (dyn Error + 'static) > { match self { Self :: Io (e) => Some (e) , Self :: InvalidArchive (_) | Self :: UnsupportedArchive (_) | Self :: FileNotFound | Self :: InvalidPassword => None , } } }
};
}
