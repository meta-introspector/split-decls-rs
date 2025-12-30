// Generated macro for impl_425 (impl)
macro_rules! Depcrate_msgs_messageimpl_425 {
() => {
// Module: crate::msgs::message
// Provides: {"impl_425"}
// Dependencies: {}
impl TryFrom < PlainMessage > for Message < 'static > { type Error = InvalidMessage ; fn try_from (plain : PlainMessage) -> Result < Self , Self :: Error > { Ok (Self { version : plain . version , payload : MessagePayload :: new (plain . typ , plain . version , plain . payload . bytes ()) ? . into_owned () , }) } }
};
}
