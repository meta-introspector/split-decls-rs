// Generated macro for impl_536 (impl)
macro_rules! Depcrate_web_audio_workletimpl_536 {
() => {
// Module: crate::web::audio_worklet
// Provides: {"impl_536"}
// Dependencies: {}
# [cfg (all (target_family = "wasm" , target_os = "unknown" , feature = "audio-worklet"))] impl < T > BaseAudioContextExt for T where BaseAudioContext : From < T > , T : AsRef < BaseAudioContext > , { fn register_thread < F > (self , stack_size : Option < usize > , # [allow (clippy :: min_ident_chars)] f : F ,) -> RegisterThreadFuture where F : 'static + FnOnce () + Send , { RegisterThreadFuture (audio_worklet :: register_thread (self . into () , stack_size , f)) } # [cfg (any (feature = "message" , docsrs))] fn register_thread_with_message < F , M > (self , stack_size : Option < usize > , # [allow (clippy :: min_ident_chars)] f : F , message : M ,) -> RegisterThreadFuture where F : 'static + FnOnce (M) + Send , M : 'static + MessageSend , { RegisterThreadFuture (audio_worklet :: register_thread_with_message (self . into () , stack_size , f , message ,)) } fn audio_worklet_node < P > (& self , name : & str , data : P :: Data , options : Option < & AudioWorkletNodeOptions > ,) -> Result < AudioWorkletNode , AudioWorkletNodeError < P > > where P : 'static + ExtendAudioWorkletProcessor , { audio_worklet :: audio_worklet_node (self . as_ref () , name , data , options) } }
};
}
