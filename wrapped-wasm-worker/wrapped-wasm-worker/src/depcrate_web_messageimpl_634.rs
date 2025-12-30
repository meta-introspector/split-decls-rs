// Generated macro for impl_634 (impl)
macro_rules! Depcrate_web_messageimpl_634 {
() => {
// Module: crate::web::message
// Provides: {"impl_634"}
// Dependencies: {}
impl < T : Into < JsValue > + JsCast + Transferable > MessageSend for TransferableWrapper < T > { type Send = () ; fn send < E : Extend < JsValue > > (self , transfer : & mut E) -> RawMessage < Self :: Send > { let serialize = self . 0 . into () ; transfer . extend ([serialize . clone ()]) ; RawMessage { serialize : Some (serialize) , send : None , } } fn receive (serialized : Option < JsValue > , sent : Option < Self :: Send >) -> Self { debug_assert_eq ! (sent , None , "unexpected `Send` value") ; Self (serialized . expect ("expected serialized value") . unchecked_into () ,) } }
};
}
