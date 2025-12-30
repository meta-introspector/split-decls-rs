// Generated macro for get_default (function)
macro_rules! Depcrate_dispatcherget_default {
() => {
// Module: crate::dispatcher
// Provides: {"get_default"}
// Dependencies: {}
# [doc = " Executes a closure with a reference to the current [dispatcher]."] # [doc = ""] # [doc = " [dispatcher]: super::dispatcher::Dispatch"] # [cfg (not (feature = "std"))] pub fn get_default < T , F > (mut f : F) -> T where F : FnMut (& Dispatch) -> T , { f (& get_global ()) }
};
}
