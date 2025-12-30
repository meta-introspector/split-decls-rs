// Generated macro for append_dir (function)
macro_rules! Depcrate_builderappend_dir {
() => {
// Module: crate::builder
// Provides: {"append_dir"}
// Dependencies: {}
fn append_dir (dst : & mut dyn Write , path : & Path , src_path : & Path , options : BuilderOptions ,) -> io :: Result < () > { let stat = fs :: metadata (src_path) ? ; append_fs (dst , path , & stat , options . mode , None) }
};
}
