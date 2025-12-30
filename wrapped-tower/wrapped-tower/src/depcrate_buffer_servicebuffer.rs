// Generated macro for Buffer (struct)
macro_rules! Depcrate_buffer_serviceBuffer {
() => {
// Module: crate::buffer::service
// Provides: {"Buffer"}
// Dependencies: {}
# [doc = " Adds an mpsc buffer in front of an inner service."] # [doc = ""] # [doc = " See the module documentation for more details."] # [derive (Debug)] pub struct Buffer < Req , F > { tx : PollSender < Message < Req , F > > , handle : Handle , }
};
}
