// Generated macro for ServerH3Event (enum)
macro_rules! Depcrate_http3_driver_serverServerH3Event {
() => {
// Module: crate::http3::driver::server
// Provides: {"ServerH3Event"}
// Dependencies: {}
# [doc = " Events produced by [ServerH3Driver]."] # [derive (Debug)] pub enum ServerH3Event { Core (H3Event) , Headers { incoming_headers : IncomingH3Headers , # [doc = " The latest PRIORITY_UPDATE frame value, if any."] priority : Option < RawPriorityValue > , } , }
};
}
