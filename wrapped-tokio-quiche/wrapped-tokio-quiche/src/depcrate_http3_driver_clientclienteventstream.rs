// Generated macro for ClientEventStream (type)
macro_rules! Depcrate_http3_driver_clientClientEventStream {
() => {
// Module: crate::http3::driver::client
// Provides: {"ClientEventStream"}
// Dependencies: {}
# [doc = " Receives [`ClientH3Event`]s from a [ClientH3Driver]. This is the control"] # [doc = " stream which describes what is happening on the connection, but does not"] # [doc = " transfer data."] pub type ClientEventStream = mpsc :: UnboundedReceiver < ClientH3Event > ;
};
}
