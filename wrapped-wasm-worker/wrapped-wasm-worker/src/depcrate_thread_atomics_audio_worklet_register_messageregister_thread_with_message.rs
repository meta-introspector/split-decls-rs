// Generated macro for register_thread_with_message (function)
macro_rules! Depcrate_thread_atomics_audio_worklet_register_messageregister_thread_with_message {
() => {
// Module: crate::thread::atomics::audio_worklet::register::message
// Provides: {"register_thread_with_message"}
// Dependencies: {}
# [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::BaseAudioContextExt::register_thread_with_message()`]."] pub (in super :: super :: super :: super) fn register_thread_with_message < F , M > (context : BaseAudioContext , stack_size : Option < usize > , task : F , message : M ,) -> RegisterThreadFuture where F : 'static + FnOnce (M) + Send , M : 'static + MessageSend , { let mut transfer_builder = ArrayBuilder :: new () ; let raw_message = message . send (& mut transfer_builder) ; let transfer = transfer_builder . finish () ; let message = raw_message . serialize . map (| serialize | MessageState { serialize , transfer , }) ; super :: register_thread_internal (context , stack_size , move | message | { let message = (! message . is_undefined ()) . then_some (message) ; let message = M :: receive (message , raw_message . send) ; task (message) ; } , message ,) }
};
}
