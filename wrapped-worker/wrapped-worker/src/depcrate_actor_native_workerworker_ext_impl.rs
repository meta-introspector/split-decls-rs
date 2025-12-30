// Generated macro for worker_ext_impl (macro)
macro_rules! Depcrate_actor_native_workerworker_ext_impl {
() => {
// Module: crate::actor::native_worker
// Provides: {"worker_ext_impl"}
// Dependencies: {}
macro_rules ! worker_ext_impl { ($ ($ type : path) ,+) => { $ (impl NativeWorkerExt for $ type { fn set_on_packed_message < T , CODEC , F > (& self , handler : F) where T : Serialize + for <'de > Deserialize <'de >, CODEC : Codec , F : 'static + Fn (T) { let handler = move | message : MessageEvent | { let msg = CODEC :: decode (message . data ()) ; handler (msg) ; } ; let closure = Closure :: wrap (Box :: new (handler) as Box < dyn Fn (MessageEvent) >) . into_js_value () ; self . set_onmessage (Some (closure . as_ref () . unchecked_ref ())) ; } fn post_packed_message < T , CODEC > (& self , data : T) where T : Serialize + for <'de > Deserialize <'de >, CODEC : Codec { self . post_message (& CODEC :: encode (data)) . expect_throw ("failed to post message") ; } }) + } ; }
};
}
