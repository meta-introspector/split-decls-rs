// Generated macro for get_current (function)
macro_rules! Depcrate_dispatcherget_current {
() => {
// Module: crate::dispatcher
// Provides: {"get_current"}
// Dependencies: {}
# [doc = " Executes a closure with a reference to the current [dispatcher]."] # [doc = ""] # [doc = " [dispatcher]: super::dispatcher::Dispatch"] # [cfg (not (feature = "std"))] # [doc (hidden)] pub fn get_current < T > (f : impl FnOnce (& Dispatch) -> T) -> Option < T > { Some (f (get_global ())) }
};
}
