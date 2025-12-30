// Generated macro for ClientH3Driver (type)
macro_rules! Depcrate_http3_driver_clientClientH3Driver {
() => {
// Module: crate::http3::driver::client
// Provides: {"ClientH3Driver"}
// Dependencies: {}
# [doc = " An [H3Driver] for a client-side HTTP/3 connection. See [H3Driver] for"] # [doc = " details. Emits [`ClientH3Event`]s and expects [`ClientH3Command`]s for"] # [doc = " control."] pub type ClientH3Driver = H3Driver < ClientHooks > ;
};
}
