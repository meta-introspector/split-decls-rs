// Generated macro for ServerH3Driver (type)
macro_rules! Depcrate_http3_driver_serverServerH3Driver {
() => {
// Module: crate::http3::driver::server
// Provides: {"ServerH3Driver"}
// Dependencies: {}
# [doc = " An [H3Driver] for a server-side HTTP/3 connection. See [H3Driver] for"] # [doc = " details. Emits [`ServerH3Event`]s and expects [`ServerH3Command`]s for"] # [doc = " control."] pub type ServerH3Driver = H3Driver < ServerHooks > ;
};
}
