// Generated macro for impl_538 (impl)
macro_rules! Depcrate_web_audio_workletimpl_538 {
() => {
// Module: crate::web::audio_worklet
// Provides: {"impl_538"}
// Dependencies: {}
impl < P > Debug for AudioWorkletNodeError < P > where P : ExtendAudioWorkletProcessor , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . debug_struct ("AudioWorkletNodeError") . field ("data" , & any :: type_name :: < P :: Data > ()) . field ("error" , & self . error) . finish () } }
};
}
