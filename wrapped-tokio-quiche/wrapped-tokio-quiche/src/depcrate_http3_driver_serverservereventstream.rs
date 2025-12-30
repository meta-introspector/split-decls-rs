// Generated macro for ServerEventStream (type)
macro_rules! Depcrate_http3_driver_serverServerEventStream {
() => {
// Module: crate::http3::driver::server
// Provides: {"ServerEventStream"}
// Dependencies: {}
# [doc = " Receives [`ServerH3Event`]s from a [ServerH3Driver]. This is the control"] # [doc = " stream which describes what is happening on the connection, but does not"] # [doc = " transfer data."] pub type ServerEventStream = mpsc :: UnboundedReceiver < ServerH3Event > ;
};
}
