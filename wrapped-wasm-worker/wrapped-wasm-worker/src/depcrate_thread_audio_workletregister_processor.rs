// Generated macro for register_processor (function)
macro_rules! Depcrate_thread_audio_workletregister_processor {
() => {
// Module: crate::thread::audio_worklet
// Provides: {"register_processor"}
// Dependencies: {}
# [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::AudioWorkletGlobalScopeExt::register_processor_ext()`]."] pub (crate) fn register_processor < P : 'static + ExtendAudioWorkletProcessor > (name : & str ,) -> Result < () , Error > { if audio_worklet :: is_main_thread () { Err (Error :: new (ErrorKind :: Unsupported , "thread was not spawned by `web-thread`" ,)) } else { audio_worklet :: register_processor :: < P > (name) } }
};
}
