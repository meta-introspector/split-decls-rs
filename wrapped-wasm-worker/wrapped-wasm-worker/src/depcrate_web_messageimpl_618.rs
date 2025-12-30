// Generated macro for impl_618 (impl)
macro_rules! Depcrate_web_messageimpl_618 {
() => {
// Module: crate::web::message
// Provides: {"impl_618"}
// Dependencies: {}
impl < T : Into < JsValue > + JsCast + Serializable > MessageSend for SerializableWrapper < T > { type Send = () ; fn send < E : Extend < JsValue > > (self , _ : & mut E) -> RawMessage < Self :: Send > { RawMessage { serialize : Some (self . 0 . into ()) , send : None , } } fn receive (serialized : Option < JsValue > , sent : Option < Self :: Send >) -> Self { debug_assert_eq ! (sent , None , "unexpected `Send` value") ; Self (serialized . expect ("expected serialized value") . unchecked_into () ,) } }
};
}
