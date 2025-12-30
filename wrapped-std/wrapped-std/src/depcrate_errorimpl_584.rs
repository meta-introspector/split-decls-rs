// Generated macro for impl_584 (impl)
macro_rules! Depcrate_errorimpl_584 {
() => {
// Module: crate::error
// Provides: {"impl_584"}
// Dependencies: {}
# [unstable (feature = "error_reporter" , issue = "90172")] impl < E > From < E > for Report < E > where E : Error , { fn from (error : E) -> Self { Report { error , show_backtrace : false , pretty : false } } }
};
}
