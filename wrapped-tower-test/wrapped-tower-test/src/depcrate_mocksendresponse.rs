// Generated macro for SendResponse (struct)
macro_rules! Depcrate_mockSendResponse {
() => {
// Module: crate::mock
// Provides: {"SendResponse"}
// Dependencies: {}
# [doc = " Send a response in reply to a received request."] # [derive (Debug)] pub struct SendResponse < T > { tx : oneshot :: Sender < Result < T , Error > > , }
};
}
