// Generated macro for audio_worklet_node (function)
macro_rules! Depcrate_thread_audio_workletaudio_worklet_node {
() => {
// Module: crate::thread::audio_worklet
// Provides: {"audio_worklet_node"}
// Dependencies: {}
# [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::BaseAudioContextExt::audio_worklet_node()`]."] pub (crate) fn audio_worklet_node < P : 'static + ExtendAudioWorkletProcessor > (context : & BaseAudioContext , name : & str , data : P :: Data , options : Option < & AudioWorkletNodeOptions > ,) -> Result < AudioWorkletNode , AudioWorkletNodeError < P > > { if audio_worklet :: is_registered (context) { audio_worklet :: audio_worklet_node (context , name , data , options) } else { Err (AudioWorkletNodeError { data , error : Error :: new (ErrorKind :: Other , "`register_thread()` has to be called on this context first" ,) , }) } }
};
}
