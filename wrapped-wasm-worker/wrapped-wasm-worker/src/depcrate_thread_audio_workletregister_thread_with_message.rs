// Generated macro for register_thread_with_message (function)
macro_rules! Depcrate_thread_audio_workletregister_thread_with_message {
() => {
// Module: crate::thread::audio_worklet
// Provides: {"register_thread_with_message"}
// Dependencies: {}
# [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::BaseAudioContextExt::register_thread_with_message()`]."] # [cfg (feature = "message")] pub (crate) fn register_thread_with_message < F , M > (context : BaseAudioContext , stack_size : Option < usize > , task : F , message : M ,) -> RegisterThreadFuture where F : 'static + FnOnce (M) + Send , M : 'static + MessageSend , { RegisterThreadFuture (if super :: has_spawn_support () { audio_worklet :: register_thread_with_message (context , stack_size , task , message) } else { audio_worklet :: RegisterThreadFuture :: error (Error :: new (ErrorKind :: Unsupported , "operation not supported on this platform without the atomics target feature and \
			 cross-origin isolation" ,)) }) }
};
}
