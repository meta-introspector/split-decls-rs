// Generated macro for impl_209 (impl)
macro_rules! Depcrate_hedge_delayimpl_209 {
() => {
// Module: crate::hedge::delay
// Provides: {"impl_209"}
// Dependencies: {}
impl < Request , F > State < Request , F > { fn delaying (delay : tokio :: time :: Sleep , req : Option < Request >) -> Self { Self :: Delaying { delay , req } } fn called (fut : F) -> Self { Self :: Called { fut } } }
};
}
