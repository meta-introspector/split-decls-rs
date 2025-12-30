// Generated macro for register_processor (function)
macro_rules! Depcrate_thread_atomics_audio_worklet_processorregister_processor {
() => {
// Module: crate::thread::atomics::audio_worklet::processor
// Provides: {"register_processor"}
// Dependencies: {}
# [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::AudioWorkletGlobalScopeExt::register_processor_ext()`]."] pub (in super :: super :: super) fn register_processor < P : 'static + ExtendAudioWorkletProcessor > (name : & str ,) -> Result < () , Error > { let name = JsString :: from_code_point (name . chars () . map (u32 :: from) . collect :: < Vec < _ > > () . as_slice ()) . expect ("found invalid Unicode") ; __web_thread_register_processor (name , __WebThreadProcessorConstructor (Box :: new (ProcessorConstructorWrapper :: < P > (PhantomData))) ,) . map_err (| error | super :: super :: error_from_exception (error . into ())) }
};
}
