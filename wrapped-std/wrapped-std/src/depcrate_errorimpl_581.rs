// Generated macro for impl_581 (impl)
macro_rules! Depcrate_errorimpl_581 {
() => {
// Module: crate::error
// Provides: {"impl_581"}
// Dependencies: {}
impl < E > Report < E > where Report < E > : From < E > , { # [doc = " Creates a new `Report` from an input error."] # [unstable (feature = "error_reporter" , issue = "90172")] pub fn new (error : E) -> Report < E > { Self :: from (error) } }
};
}
