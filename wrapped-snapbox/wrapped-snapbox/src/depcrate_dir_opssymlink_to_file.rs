// Generated macro for symlink_to_file (function)
macro_rules! Depcrate_dir_opssymlink_to_file {
() => {
// Module: crate::dir::ops
// Provides: {"symlink_to_file"}
// Dependencies: {}
# [cfg (not (windows))] fn symlink_to_file (link : & std :: path :: Path , target : & std :: path :: Path) -> Result < () , std :: io :: Error > { std :: os :: unix :: fs :: symlink (target , link) }
};
}
