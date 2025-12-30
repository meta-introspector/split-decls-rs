// Generated macro for RequestSender (struct)
macro_rules! Depcrate_http3_driverRequestSender {
() => {
// Module: crate::http3::driver
// Provides: {"RequestSender"}
// Dependencies: {}
# [doc = " Sends [`H3Command`]s to an [H3Driver]. The sender is typed and internally"] # [doc = " wraps instances of `T` in the appropriate `H3Command` variant."] pub struct RequestSender < C , T > { sender : UnboundedSender < C > , _r : PhantomData < fn () -> T > , }
};
}
