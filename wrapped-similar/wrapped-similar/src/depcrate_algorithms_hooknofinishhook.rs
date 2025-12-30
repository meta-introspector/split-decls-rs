// Generated macro for NoFinishHook (struct)
macro_rules! Depcrate_algorithms_hookNoFinishHook {
() => {
// Module: crate::algorithms::hook
// Provides: {"NoFinishHook"}
// Dependencies: {}
# [doc = " Wrapper [`DiffHook`] that prevents calls to [`DiffHook::finish`]."] # [doc = ""] # [doc = " This hook is useful in situations where diff hooks are composed but you"] # [doc = " want to prevent that the finish hook method is called."] pub struct NoFinishHook < D : DiffHook > (D) ;
};
}
