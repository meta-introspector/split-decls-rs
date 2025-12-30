// Generated macro for scope_async (function)
macro_rules! Depcrate_thread_scopescope_async {
() => {
// Module: crate::thread::scope
// Provides: {"scope_async"}
// Dependencies: {}
# [doc = " Implementation for [`crate::web::scope_async()`]."] pub (crate) fn scope_async < 'scope , 'env : 'scope , F1 , F2 , T > (task : F1 ,) -> ScopeFuture < 'scope , 'env , F2 , T > where F1 : FnOnce (& 'scope Scope < 'scope , 'env >) -> F2 , F2 : Future < Output = T > , { let scope = Box :: pin (Scope { this : r#impl :: Scope :: new () , _scope : PhantomData , _env : PhantomData , }) ; let task = task (unsafe { mem :: transmute :: < & Scope < '_ , '_ > , & Scope < '_ , '_ > > (scope . deref ()) }) ; ScopeFuture :: new (task , scope) }
};
}
