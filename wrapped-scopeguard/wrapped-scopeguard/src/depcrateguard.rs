// Generated macro for guard (function)
macro_rules! Depcrateguard {
() => {
// Module: crate
// Provides: {"guard"}
// Dependencies: {}
# [doc = " Create a new `ScopeGuard` owning `v` and with deferred closure `dropfn`."] # [inline] # [must_use] pub fn guard < T , F > (v : T , dropfn : F) -> ScopeGuard < T , F , Always > where F : FnOnce (T) , { ScopeGuard :: with_strategy (v , dropfn) }
};
}
