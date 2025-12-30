// Generated macro for submit (function)
macro_rules! Depcratesubmit {
() => {
// Module: crate
// Provides: {"submit"}
// Dependencies: {}
# [doc = " Submit the closure to the default thread pool."] # [doc = ""] # [doc = " * The closure must have `'static` lifetime as the thread may outlive the lifetime in which `submit` is called."] # [doc = " * The closure must be `Send` as it will be sent to another thread for execution."] pub fn submit < F : FnOnce () + Send + 'static > (f : F) { unsafe { try_submit (core :: ptr :: null () , f) ; } }
};
}
