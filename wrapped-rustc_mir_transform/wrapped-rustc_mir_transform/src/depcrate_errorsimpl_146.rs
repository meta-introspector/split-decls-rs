// Generated macro for impl_146 (impl)
macro_rules! Depcrate_errorsimpl_146 {
() => {
// Module: crate::errors
// Provides: {"impl_146"}
// Dependencies: {}
impl AssertLintKind { pub (crate) fn lint (& self) -> & 'static Lint { match self { AssertLintKind :: ArithmeticOverflow => lint :: builtin :: ARITHMETIC_OVERFLOW , AssertLintKind :: UnconditionalPanic => lint :: builtin :: UNCONDITIONAL_PANIC , } } }
};
}
