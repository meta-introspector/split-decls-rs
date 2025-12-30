// Generated macro for path_key (function)
macro_rules! Depcrate_dist_cachepath_key {
() => {
// Module: crate::dist::cache
// Provides: {"path_key"}
// Dependencies: {}
# [cfg (feature = "dist-client")] fn path_key (path : & Path) -> Result < String > { file_key (fs :: File :: open (path) ?) }
};
}
