// Generated macro for create (function)
macro_rules! Depcrate_dircreate {
() => {
// Module: crate::dir
// Provides: {"create"}
// Dependencies: {}
pub (crate) fn create (path : PathBuf , permissions : Option < & std :: fs :: Permissions > , disable_cleanup : bool ,) -> io :: Result < TempDir > { imp :: create (path , permissions , disable_cleanup) }
};
}
