// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl < T , F , S > ScopeGuard < T , F , S > where F : FnOnce (T) , S : Strategy , { # [doc = " Create a `ScopeGuard` that owns `v` (accessible through deref) and calls"] # [doc = " `dropfn` when its destructor runs."] # [doc = ""] # [doc = " The `Strategy` decides whether the scope guard's closure should run."] # [inline] # [must_use] pub fn with_strategy (v : T , dropfn : F) -> ScopeGuard < T , F , S > { ScopeGuard { value : ManuallyDrop :: new (v) , dropfn : ManuallyDrop :: new (dropfn) , strategy : PhantomData , } } # [doc = " “Defuse” the guard and extract the value without calling the closure."] # [doc = ""] # [doc = " ```"] # [doc = " extern crate scopeguard;"] # [doc = ""] # [doc = " use scopeguard::{guard, ScopeGuard};"] # [doc = ""] # [doc = " fn conditional() -> bool { true }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let mut guard = guard(Vec::new(), |mut v| v.clear());"] # [doc = "     guard.push(1);"] # [doc = ""] # [doc = "     if conditional() {"] # [doc = "         // a condition maybe makes us decide to"] # [doc = "         // “defuse” the guard and get back its inner parts"] # [doc = "         let value = ScopeGuard::into_inner(guard);"] # [doc = "     } else {"] # [doc = "         // guard still exists in this branch"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [inline] pub fn into_inner (guard : Self) -> T { let mut guard = ManuallyDrop :: new (guard) ; unsafe { let value = ptr :: read (& * guard . value) ; ManuallyDrop :: drop (& mut guard . dropfn) ; value } } }
};
}
