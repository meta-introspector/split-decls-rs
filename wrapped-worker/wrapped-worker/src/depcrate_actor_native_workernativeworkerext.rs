// Generated macro for NativeWorkerExt (trait)
macro_rules! Depcrate_actor_native_workerNativeWorkerExt {
() => {
// Module: crate::actor::native_worker
// Provides: {"NativeWorkerExt"}
// Dependencies: {}
pub (crate) trait NativeWorkerExt { fn set_on_packed_message < T , CODEC , F > (& self , handler : F) where T : Serialize + for < 'de > Deserialize < 'de > , CODEC : Codec , F : 'static + Fn (T) ; fn post_packed_message < T , CODEC > (& self , data : T) where T : Serialize + for < 'de > Deserialize < 'de > , CODEC : Codec ; }
};
}
