// Generated macro for ClientRequestSender (type)
macro_rules! Depcrate_http3_driver_clientClientRequestSender {
() => {
// Module: crate::http3::driver::client
// Provides: {"ClientRequestSender"}
// Dependencies: {}
# [doc = " A [RequestSender] to send HTTP requests over a [ClientH3Driver]'s"] # [doc = " connection."] pub type ClientRequestSender = RequestSender < ClientH3Command , NewClientRequest > ;
};
}
