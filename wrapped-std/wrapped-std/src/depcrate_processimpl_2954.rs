// Generated macro for impl_2954 (impl)
macro_rules! Depcrate_processimpl_2954 {
() => {
// Module: crate::process
// Provides: {"impl_2954"}
// Dependencies: {}
impl FromInner < (imp :: Process , imp :: StdioPipes) > for Child { fn from_inner ((handle , io) : (imp :: Process , imp :: StdioPipes)) -> Child { Child { handle , stdin : io . stdin . map (ChildStdin :: from_inner) , stdout : io . stdout . map (ChildStdout :: from_inner) , stderr : io . stderr . map (ChildStderr :: from_inner) , } } }
};
}
