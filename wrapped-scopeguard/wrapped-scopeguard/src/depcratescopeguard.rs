// Generated macro for ScopeGuard (struct)
macro_rules! DepcrateScopeGuard {
() => {
// Module: crate
// Provides: {"ScopeGuard"}
// Dependencies: {}
# [doc = " `ScopeGuard` is a scope guard that may own a protected value."] # [doc = ""] # [doc = " If you place a guard in a local variable, the closure can"] # [doc = " run regardless how you leave the scope — through regular return or panic"] # [doc = " (except if panic or other code aborts; so as long as destructors run)."] # [doc = " It is run only once."] # [doc = ""] # [doc = " The `S` parameter for [`Strategy`](trait.Strategy.html) determines if"] # [doc = " the closure actually runs."] # [doc = ""] # [doc = " The guard's closure will be called with the held value in the destructor."] # [doc = ""] # [doc = " The `ScopeGuard` implements `Deref` so that you can access the inner value."] pub struct ScopeGuard < T , F , S = Always > where F : FnOnce (T) , S : Strategy , { value : ManuallyDrop < T > , dropfn : ManuallyDrop < F > , strategy : PhantomData < fn (S) -> S > , }
};
}
