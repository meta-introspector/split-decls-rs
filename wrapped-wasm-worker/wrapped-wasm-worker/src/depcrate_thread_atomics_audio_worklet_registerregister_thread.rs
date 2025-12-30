// Generated macro for register_thread (function)
macro_rules! Depcrate_thread_atomics_audio_worklet_registerregister_thread {
() => {
// Module: crate::thread::atomics::audio_worklet::register
// Provides: {"register_thread"}
// Dependencies: {}
# [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::BaseAudioContextExt::register_thread()`]."] pub (in super :: super :: super) fn register_thread < F > (context : BaseAudioContext , stack_size : Option < usize > , task : F ,) -> RegisterThreadFuture where F : 'static + FnOnce () + Send , { register_thread_internal (context , stack_size , | _ | task () , # [cfg (feature = "message")] None ,) }
};
}
