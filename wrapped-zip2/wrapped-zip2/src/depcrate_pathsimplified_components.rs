// Generated macro for simplified_components (function)
macro_rules! Depcrate_pathsimplified_components {
() => {
// Module: crate::path
// Provides: {"simplified_components"}
// Dependencies: {}
# [doc = " Simplify a path by removing the prefix and parent directories and only return normal components"] pub (crate) fn simplified_components (input : & Path) -> Option < Vec < & OsStr > > { let mut out = Vec :: new () ; for component in input . components () { match component { Component :: Prefix (_) | Component :: RootDir => return None , Component :: ParentDir => { out . pop () ? ; } Component :: Normal (_) => out . push (component . as_os_str ()) , Component :: CurDir => () , } } Some (out) }
};
}
