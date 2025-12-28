macro_rules! deps {
    () => {
        TP_CALLBACK_ENVIRON_V3!();
        Pool!();
        Scope!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Pool { # [doc = " Creates a new `Pool` object."] pub fn new () -> Self { let mut e = TP_CALLBACK_ENVIRON_V3 { Version : 3 , CallbackPriority : TP_CALLBACK_PRIORITY_NORMAL , Size : core :: mem :: size_of :: < TP_CALLBACK_ENVIRON_V3 > () as u32 , .. Default :: default () } ; unsafe { e . Pool = check (CreateThreadpool (core :: ptr :: null ())) ; e . CleanupGroup = check (CreateThreadpoolCleanupGroup ()) ; } Self (Box :: new (e)) } # [doc = " Convenience function for creating a new pool and calling [`scope`][Self::scope]."] pub fn with_scope < 'env , F > (f : F) where F : for < 'scope > FnOnce (& 'scope Scope < 'scope , 'env >) , { let pool = Pool :: new () ; pool . scope (f) ; } # [doc = " Sets the thread limits for the `Pool` object."] pub fn set_thread_limits (& self , min : u32 , max : u32) { unsafe { check (SetThreadpoolThreadMinimum (self . 0 . Pool , min)) ; SetThreadpoolThreadMaximum (self . 0 . Pool , max) ; } } # [doc = " Submit the closure to the thread pool."] # [doc = ""] # [doc = " * The closure must have `'static` lifetime as the thread may outlive the lifetime in which `submit` is called."] # [doc = " * The closure must be `Send` as it will be sent to another thread for execution."] pub fn submit < F : FnOnce () + Send + 'static > (& self , f : F) { unsafe { try_submit (& * self . 0 , f) ; } } # [doc = " Create a scope for submitting closures."] # [doc = ""] # [doc = " Within this scope local variables can be sent to the pool thread for execution."] # [doc = " This is possible because `scope` will wait for all submitted closures to finish before returning,"] # [doc = " Note however that it will also wait for closures that were submitted from other threads."] pub fn scope < 'env , F > (& self , f : F) where F : for < 'scope > FnOnce (& 'scope Scope < 'scope , 'env >) , { struct DropGuard < 'a > (& 'a Pool) ; impl Drop for DropGuard < '_ > { fn drop (& mut self) { self . 0 . join () ; } } let _guard = DropGuard (self) ; let scope = Scope { pool : self , env : PhantomData , scope : PhantomData , } ; f (& scope) ; } # [doc = " Waits for all submissions to finish."] # [doc = ""] # [doc = " Dropping the `Pool` will also wait for all submissions to finish."] pub fn join (& self) { unsafe { CloseThreadpoolCleanupGroupMembers (self . 0 . CleanupGroup , 0 , core :: ptr :: null_mut ()) ; } } }
    };
}

impl_25!()