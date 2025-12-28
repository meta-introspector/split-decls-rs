macro_rules! halt_unwinding {
    () => {
        # [doc = " Executes `f` and captures any panic, translating that panic into a"] # [doc = " `Err` result. The assumption is that any panic will be propagated"] # [doc = " later with `resume_unwinding`, and hence `f` can be treated as"] # [doc = " exception safe."] pub (super) fn halt_unwinding < F , R > (func : F) -> thread :: Result < R > where F : FnOnce () -> R , { panic :: catch_unwind (AssertUnwindSafe (func)) }
    };
}

halt_unwinding!()