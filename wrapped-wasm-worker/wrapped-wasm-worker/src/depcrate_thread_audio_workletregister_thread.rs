// Generated macro for register_thread (function)
macro_rules! Depcrate_thread_audio_workletregister_thread {
() => {
// Module: crate::thread::audio_worklet
// Provides: {"register_thread"}
// Dependencies: {}
# [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::BaseAudioContextExt::register_thread()`]."] pub (crate) fn register_thread < F > (context : BaseAudioContext , stack_size : Option < usize > , task : F ,) -> RegisterThreadFuture where F : 'static + FnOnce () + Send , { RegisterThreadFuture (if super :: has_spawn_support () { audio_worklet :: register_thread (context , stack_size , task) } else { audio_worklet :: RegisterThreadFuture :: error (Error :: new (ErrorKind :: Unsupported , "operation not supported on this platform without the atomics target feature and \
			 cross-origin isolation" ,)) }) }
};
}
