// Generated macro for guard_on_unwind (function)
macro_rules! Depcrateguard_on_unwind {
() => {
// Module: crate
// Provides: {"guard_on_unwind"}
// Dependencies: {}
# [doc = " Create a new `ScopeGuard` owning `v` and with deferred closure `dropfn`."] # [doc = ""] # [doc = " Requires crate feature `use_std`."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " For performance reasons, or to emulate “only run guard on unwind” in"] # [doc = " no-std environments, we can also use the default guard and simply manually"] # [doc = " defuse it at the end of scope like the following example. (The performance"] # [doc = " reason would be if the [`OnUnwind`]'s call to [std::thread::panicking()] is"] # [doc = " an issue.)"] # [doc = ""] # [doc = " ```"] # [doc = " extern crate scopeguard;"] # [doc = ""] # [doc = " use scopeguard::ScopeGuard;"] # [doc = " # fn main() {"] # [doc = " {"] # [doc = "     let guard = scopeguard::guard((), |_| {});"] # [doc = ""] # [doc = "     // rest of the code here"] # [doc = ""] # [doc = "     // we reached the end of scope without unwinding - defuse it"] # [doc = "     ScopeGuard::into_inner(guard);"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] # [cfg (feature = "use_std")] # [inline] # [must_use] pub fn guard_on_unwind < T , F > (v : T , dropfn : F) -> ScopeGuard < T , F , OnUnwind > where F : FnOnce (T) , { ScopeGuard :: with_strategy (v , dropfn) }
};
}
