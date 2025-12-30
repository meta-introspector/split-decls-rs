// Generated macro for impl_422 (impl)
macro_rules! Depcrate_msgs_messageimpl_422 {
() => {
// Module: crate::msgs::message
// Provides: {"impl_422"}
// Dependencies: {}
impl From < Message < '_ > > for PlainMessage { fn from (msg : Message < '_ >) -> Self { let typ = msg . payload . content_type () ; let payload = match msg . payload { MessagePayload :: ApplicationData (payload) => payload . into_owned () , _ => { let mut buf = Vec :: new () ; msg . payload . encode (& mut buf) ; Payload :: Owned (buf) } } ; Self { typ , version : msg . version , payload , } } }
};
}
