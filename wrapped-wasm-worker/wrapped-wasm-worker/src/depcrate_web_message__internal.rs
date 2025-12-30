// Generated macro for __internal (module)
macro_rules! Depcrate_web_message__internal {
() => {
// Module: crate::web::message
// Provides: {"__internal"}
// Dependencies: {}
# [doc (hidden)] # [cfg (all (target_family = "wasm" , target_os = "unknown"))] pub mod __internal { use wasm_bindgen :: { JsCast , JsValue } ; use super :: { MessageSend , RawMessage , SendWrapper , Serializable , SerializableWrapper , Transferable , TransferableWrapper , } ; macro_rules ! impl_internal { ($ name : ident for $ priority : ty , $ wrapper : ident , $ ($ bound : path) |+) => { pub trait $ name < T > { type Send ; fn __web_thread_send < E : Extend < JsValue >> (self , extend : & mut E) -> RawMessage < Self :: Send >; fn __web_thread_receive (self , serialized : Option < JsValue >, sent : Option < Self :: Send >) -> T ; } # [allow (clippy :: mut_mut)] impl < T : $ ($ bound +) +> $ name < T > for $ priority { type Send = <$ wrapper < T > as MessageSend >:: Send ; fn __web_thread_send < E : Extend < JsValue >> (self , extend : & mut E) -> RawMessage < Self :: Send > { $ wrapper (self . take () . expect ("found empty `Option` while sending")) . send (extend) } fn __web_thread_receive (self , serialized : Option < JsValue >, sent : Option < Self :: Send >) -> T { debug_assert ! (self . is_none () , "found filled `Option` while receiving") ; $ wrapper :: receive (serialized , sent) . 0 } } } ; } impl_internal ! (InternalSerializable for & mut & mut Option < T >, SerializableWrapper , Serializable | Into < JsValue > | JsCast) ; impl_internal ! (InternalTransferable for & mut Option < T >, TransferableWrapper , Transferable | Into < JsValue > | JsCast) ; impl_internal ! (InternalSend for & mut Option < T >, SendWrapper , Send) ; }
};
}
