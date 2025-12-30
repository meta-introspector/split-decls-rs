// Generated macro for guard_on_success (function)
macro_rules! Depcrateguard_on_success {
() => {
// Module: crate
// Provides: {"guard_on_success"}
// Dependencies: {}
# [doc = " Create a new `ScopeGuard` owning `v` and with deferred closure `dropfn`."] # [doc = ""] # [doc = " Requires crate feature `use_std`."] # [cfg (feature = "use_std")] # [inline] # [must_use] pub fn guard_on_success < T , F > (v : T , dropfn : F) -> ScopeGuard < T , F , OnSuccess > where F : FnOnce (T) , { ScopeGuard :: with_strategy (v , dropfn) }
};
}
