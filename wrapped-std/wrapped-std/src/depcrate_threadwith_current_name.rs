// Generated macro for with_current_name (function)
macro_rules! Depcrate_threadwith_current_name {
() => {
// Module: crate::thread
// Provides: {"with_current_name"}
// Dependencies: {}
# [doc = " Run a function with the current thread's name."] # [doc = ""] # [doc = " Modulo thread local accesses, this function is safe to call from signal"] # [doc = " handlers and in similar circumstances where allocations are not possible."] pub (crate) fn with_current_name < F , R > (f : F) -> R where F : FnOnce (Option < & str >) -> R , { try_with_current (| thread | { if let Some (thread) = thread { if let Some (name) = & thread . inner . name { return f (Some (name . as_str ())) ; } else if Some (thread . inner . id) == main_thread :: get () { return f (Some ("main")) ; } } else if let Some (main) = main_thread :: get () && let Some (id) = current :: id :: get () && id == main { return f (Some ("main")) ; } f (None) }) }
};
}
