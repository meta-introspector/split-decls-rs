// Generated macro for macro_61 (macro)
macro_rules! Depcrate_stream_extmacro_61 {
() => {
// Module: crate::stream_ext
// Provides: {"macro_61"}
// Dependencies: {}
cfg_time ! { pub (crate) mod timeout ; pub (crate) mod timeout_repeating ; pub use timeout :: Timeout ; pub use timeout_repeating :: TimeoutRepeating ; use tokio :: time :: { Duration , Interval } ; mod throttle ; use throttle :: { throttle , Throttle } ; mod chunks_timeout ; pub use chunks_timeout :: ChunksTimeout ; }
};
}
