// Generated macro for simplify_path (function)
macro_rules! Depcrate_dist_pkgsimplify_path {
() => {
// Module: crate::dist::pkg
// Provides: {"simplify_path"}
// Dependencies: {}
# [doc = " Simplify a path to one without any relative components, erroring if it looks"] # [doc = " like there could be any symlink complexity that means a simplified path is not"] # [doc = " equivalent to the original (see the documentation of `fs::canonicalize` for an"] # [doc = " example)."] # [doc = ""] # [doc = " So why avoid resolving symlinks? Any path that we are trying to simplify has"] # [doc = " (usually) been added to an archive because something will try access it, but"] # [doc = " resolving symlinks (be they for the actual file or directory components) can"] # [doc = " make the accessed path 'disappear' in favour of the canonical path."] pub fn simplify_path (path : & Path) -> Result < PathBuf > { SimplifyPath { resolved_symlinks : None , } . simplify (path) }
};
}
