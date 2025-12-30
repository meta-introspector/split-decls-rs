// Generated macro for impl_565 (impl)
macro_rules! Depcrate_web_messageimpl_565 {
() => {
// Module: crate::web::message
// Provides: {"impl_565"}
// Dependencies: {}
impl < const SIZE : usize , T : MessageSend > MessageSend for [T ; SIZE] { type Send = [Option < T :: Send > ; SIZE] ; fn send < E : Extend < JsValue > > (self , transfer : & mut E) -> RawMessage < Self :: Send > { let mut serialize_builder = None ; let mut empty_serialize_count = 0 ; let mut has_send = false ; let send = self . map (| message | { let message = message . send (transfer) ; if let Some (serialize) = message . serialize { serialize_builder . get_or_insert_with (| | { let mut builder = ArrayBuilder :: new () ; builder . extend (iter :: repeat (JsValue :: NULL) . take (empty_serialize_count)) ; builder }) . push (serialize) ; } else { empty_serialize_count += 1 ; } if message . send . is_some () { has_send = true ; } message . send }) ; RawMessage { serialize : serialize_builder . and_then (ArrayBuilder :: finish) . map (Array :: unchecked_into) , send : has_send . then_some (send) , } } fn receive (serialized : Option < JsValue > , mut sent : Option < Self :: Send >) -> Self { let serialized = serialized . map (Array :: unchecked_from_js) ; if let Some (serialized) = & serialized { debug_assert_eq ! (serialized . length () , usize_is_u32 (SIZE) , "unexpected array size during message receival") ; } array :: from_fn (| index | { let serialized = serialized . as_ref () . map (| serialized | serialized . get (usize_is_u32 (index))) . filter (| value | ! value . is_null ()) ; let sent = sent . as_mut () . and_then (| sent | sent . get_mut (index)) . and_then (Option :: take) ; T :: receive (serialized , sent) }) } }
};
}
