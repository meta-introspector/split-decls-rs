// Generated macro for impl_124 (impl)
macro_rules! Depcrate_dispatcherimpl_124 {
() => {
// Module: crate::dispatcher
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (feature = "std")] impl State { # [doc = " Replaces the current default dispatcher on this thread with the provided"] # [doc = " dispatcher.Any"] # [doc = ""] # [doc = " Dropping the returned `ResetGuard` will reset the default dispatcher to"] # [doc = " the previous value."] # [inline] fn set_default (new_dispatch : Dispatch) -> DefaultGuard { let prior = CURRENT_STATE . try_with (| state | { state . can_enter . set (true) ; state . default . replace (Some (new_dispatch)) }) . ok () . flatten () ; EXISTS . store (true , Ordering :: Release) ; SCOPED_COUNT . fetch_add (1 , Ordering :: Release) ; DefaultGuard (prior) } # [inline] fn enter (& self) -> Option < Entered < '_ > > { if self . can_enter . replace (false) { Some (Entered (self)) } else { None } } }
};
}
