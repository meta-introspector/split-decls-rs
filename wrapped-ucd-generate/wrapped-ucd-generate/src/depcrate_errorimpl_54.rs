// Generated macro for impl_54 (impl)
macro_rules! Depcrate_errorimpl_54 {
() => {
// Module: crate::error
// Provides: {"impl_54"}
// Dependencies: {}
impl Error { pub fn is_broken_pipe (& self) -> bool { match * self { Error :: Io (ref e) if e . kind () == io :: ErrorKind :: BrokenPipe => true , _ => false , } } }
};
}
