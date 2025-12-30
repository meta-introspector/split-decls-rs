// Generated macro for macro_929 (macro)
macro_rules! Depcrate_catch_panicmacro_929 {
() => {
// Module: crate::catch_panic
// Provides: {"macro_929"}
// Dependencies: {}
pin_project ! { # [project = KindProj] enum Kind < F , T > { Panicked { panic_err : Option < Box < dyn Any + Send + 'static >>, panic_handler : Option < T >, } , Future { # [pin] future : CatchUnwind < AssertUnwindSafe < F >>, panic_handler : Option < T >, } } }
};
}
