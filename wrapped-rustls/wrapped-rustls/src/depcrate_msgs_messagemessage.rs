// Generated macro for Message (struct)
macro_rules! Depcrate_msgs_messageMessage {
() => {
// Module: crate::msgs::message
// Provides: {"Message"}
// Dependencies: {}
# [doc = " A message with decoded payload"] # [expect (clippy :: exhaustive_structs)] # [derive (Debug)] pub struct Message < 'a > { pub version : ProtocolVersion , pub payload : MessagePayload < 'a > , }
};
}
