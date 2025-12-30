// Generated macro for impl_426 (impl)
macro_rules! Depcrate_msgs_messageimpl_426 {
() => {
// Module: crate::msgs::message
// Provides: {"impl_426"}
// Dependencies: {}
# [doc = " Parses a plaintext message into a well-typed [`Message`]."] # [doc = ""] # [doc = " A [`PlainMessage`] must contain plaintext content. Encrypted content should be stored in an"] # [doc = " [`InboundOpaqueMessage`] and decrypted before being stored into a [`PlainMessage`]."] # [doc = ""] # [doc = " [`InboundOpaqueMessage`]: crate::crypto::cipher::InboundOpaqueMessage"] impl < 'a > TryFrom < InboundPlainMessage < 'a > > for Message < 'a > { type Error = InvalidMessage ; fn try_from (plain : InboundPlainMessage < 'a >) -> Result < Self , Self :: Error > { Ok (Self { version : plain . version , payload : MessagePayload :: new (plain . typ , plain . version , plain . payload) ? , }) } }
};
}
