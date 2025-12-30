// Generated macro for impl_637 (impl)
macro_rules! Depcrate_web_messageimpl_637 {
() => {
// Module: crate::web::message
// Provides: {"impl_637"}
// Dependencies: {}
impl < T : Send > MessageSend for SendWrapper < T > { type Send = T ; fn send < E : Extend < JsValue > > (self , _ : & mut E) -> RawMessage < Self :: Send > { RawMessage { serialize : None , send : Some (self . 0) , } } fn receive (serialized : Option < JsValue > , sent : Option < Self :: Send >) -> Self { debug_assert_eq ! (serialized , None , "unexpected serialized `JsValue`") ; Self (sent . expect ("expected serialized value")) } }
};
}
