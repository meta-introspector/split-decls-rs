// Generated macro for ClientH3Event (enum)
macro_rules! Depcrate_http3_driver_clientClientH3Event {
() => {
// Module: crate::http3::driver::client
// Provides: {"ClientH3Event"}
// Dependencies: {}
# [doc = " Events produced by [ClientH3Driver]."] # [derive (Debug)] pub enum ClientH3Event { Core (H3Event) , # [doc = " Headers for the request with the given `request_id` were sent on"] # [doc = " `stream_id`. The body, if there is one, could still be sending."] NewOutboundRequest { stream_id : u64 , request_id : u64 , } , }
};
}
