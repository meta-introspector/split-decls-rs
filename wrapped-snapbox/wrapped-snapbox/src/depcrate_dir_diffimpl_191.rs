// Generated macro for impl_191 (impl)
macro_rules! Depcrate_dir_diffimpl_191 {
() => {
// Module: crate::dir::diff
// Provides: {"impl_191"}
// Dependencies: {}
impl FileType { fn as_str (self) -> & 'static str { match self { Self :: Dir => "dir" , Self :: File => "file" , Self :: Symlink => "symlink" , Self :: Unknown => "unknown" , Self :: Missing => "missing" , } } }
};
}
