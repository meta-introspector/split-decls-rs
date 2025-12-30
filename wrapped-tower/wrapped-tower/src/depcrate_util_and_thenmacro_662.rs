// Generated macro for macro_662 (macro)
macro_rules! Depcrate_util_and_thenmacro_662 {
() => {
// Module: crate::util::and_then
// Provides: {"macro_662"}
// Dependencies: {}
pin_project_lite :: pin_project ! { # [doc = " Response future from [`AndThen`] services."] # [doc = ""] # [doc = " [`AndThen`]: crate::util::AndThen"] pub struct AndThenFuture < F1 , F2 : TryFuture , N > { # [pin] inner : future :: AndThen < future :: ErrInto < F1 , F2 :: Error >, F2 , N >, } }
};
}
