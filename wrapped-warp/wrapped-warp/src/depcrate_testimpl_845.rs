// Generated macro for impl_845 (impl)
macro_rules! Depcrate_testimpl_845 {
() => {
// Module: crate::test
// Provides: {"impl_845"}
// Dependencies: {}
# [cfg (feature = "websocket")] impl WsError { fn new < E : Into < Box < dyn StdError + Send + Sync > > > (cause : E) -> Self { WsError { cause : cause . into () , } } }
};
}
