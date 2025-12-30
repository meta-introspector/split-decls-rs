// Generated macro for DecorateAsyncRead (trait)
macro_rules! Depcrate_compression_utilsDecorateAsyncRead {
() => {
// Module: crate::compression_utils
// Provides: {"DecorateAsyncRead"}
// Dependencies: {}
# [doc = " Trait for applying some decorator to an `AsyncRead`"] pub (crate) trait DecorateAsyncRead { type Input : AsyncRead ; type Output : AsyncRead ; # [doc = " Apply the decorator"] fn apply (input : Self :: Input , quality : CompressionLevel) -> Self :: Output ; # [doc = " Get a pinned mutable reference to the original input."] # [doc = ""] # [doc = " This is necessary to implement `Body::poll_trailers`."] fn get_pin_mut (pinned : Pin < & mut Self :: Output >) -> Pin < & mut Self :: Input > ; }
};
}
