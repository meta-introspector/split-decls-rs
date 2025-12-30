// Generated macro for RawMessage (struct)
macro_rules! Depcrate_web_messageRawMessage {
() => {
// Module: crate::web::message
// Provides: {"RawMessage"}
// Dependencies: {}
# [doc = " Contains data necessary to send [`MessageSend`] to another thread."] # [derive (Debug , PartialEq)] pub struct RawMessage < T > { # [doc = " Value to be [serialized](https://developer.mozilla.org/en-US/docs/Glossary/Serializable_object)."] pub serialize : Option < JsValue > , # [doc = " [`Send`] value."] pub send : Option < T > , }
};
}
