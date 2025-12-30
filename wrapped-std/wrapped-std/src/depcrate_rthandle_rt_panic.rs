// Generated macro for handle_rt_panic (function)
macro_rules! Depcrate_rthandle_rt_panic {
() => {
// Module: crate::rt
// Provides: {"handle_rt_panic"}
// Dependencies: {}
fn handle_rt_panic < T > (e : Box < dyn Any + Send >) -> T { mem :: forget (e) ; rtabort ! ("initialization or cleanup bug") ; }
};
}
