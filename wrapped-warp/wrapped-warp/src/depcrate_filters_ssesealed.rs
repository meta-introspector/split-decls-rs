// Generated macro for sealed (module)
macro_rules! Depcrate_filters_ssesealed {
() => {
// Module: crate::filters::sse
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { use super :: * ; # [doc = " SSE error type"] # [derive (Debug)] pub struct SseError ; impl fmt :: Display for SseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "sse error") } } impl StdError for SseError { } }
};
}
