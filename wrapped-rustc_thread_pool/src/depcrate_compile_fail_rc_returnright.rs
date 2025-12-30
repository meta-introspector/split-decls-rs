// Generated macro for right (module)
macro_rules! Depcrate_compile_fail_rc_returnright {
() => {
// Module: crate::compile_fail::rc_return
// Provides: {"right"}
// Dependencies: {}
# [doc = " ```compile_fail,E0277\n\nuse std::rc::Rc;\n\nrustc_thread_pool::join(|| (), || Rc::new(23)); //~ ERROR\n\n``` "] mod right { }
};
}
