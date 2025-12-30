// Generated macro for impl_321 (impl)
macro_rules! Depcrate_move_pathsimpl_321 {
() => {
// Module: crate::move_paths
// Provides: {"impl_321"}
// Dependencies: {}
impl Init { pub fn span < 'tcx > (& self , body : & Body < 'tcx >) -> Span { match self . location { InitLocation :: Argument (local) => body . local_decls [local] . source_info . span , InitLocation :: Statement (location) => body . source_info (location) . span , } } }
};
}
