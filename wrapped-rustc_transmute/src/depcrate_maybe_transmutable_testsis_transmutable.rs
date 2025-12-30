// Generated macro for is_transmutable (function)
macro_rules! Depcrate_maybe_transmutable_testsis_transmutable {
() => {
// Module: crate::maybe_transmutable::tests
// Provides: {"is_transmutable"}
// Dependencies: {}
fn is_transmutable < R : Representation + Clone > (src : & R , dst : & R , assume : Assume ,) -> crate :: Answer < ! , ! > { let src = src . clone () ; let dst = dst . clone () ; R :: is_transmutable (src , dst , assume) }
};
}
