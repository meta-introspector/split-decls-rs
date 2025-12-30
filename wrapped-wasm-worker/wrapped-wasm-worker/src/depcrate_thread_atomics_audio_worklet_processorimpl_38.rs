// Generated macro for impl_38 (impl)
macro_rules! Depcrate_thread_atomics_audio_worklet_processorimpl_38 {
() => {
// Module: crate::thread::atomics::audio_worklet::processor
// Provides: {"impl_38"}
// Dependencies: {}
impl < P : 'static + ExtendAudioWorkletProcessor > ProcessorConstructor for ProcessorConstructorWrapper < P > { fn instantiate (& mut self , this : web_sys :: AudioWorkletProcessor , options : AudioWorkletNodeOptions ,) -> __WebThreadProcessor { let mut processor_data = None ; if let Some (processor_options) = options . get_processor_options () { let processor_options : ProcessorOptions = processor_options . unchecked_into () ; if let Some (data) = processor_options . data () { let data : Data = * unsafe { Box :: < Data > :: from_raw (data . as_ptr ()) } ; if data . type_id == TypeId :: of :: < P > () { processor_data = Some (* data . value . downcast :: < P :: Data > () . expect ("wrong type encoded") ,) ; if data . empty { PROCESSOR_OPTIONS_PROPERTY_NAME . with (| name | Reflect :: delete_property (& options , name)) . expect ("expected `AudioWorkletNodeOptions` to be an `Object`") ; } else { DATA_PROPERTY_NAME . with (| name | Reflect :: delete_property (& processor_options , name)) . expect ("expected `processor_options` to be an `Object`") ; } } } } __WebThreadProcessor (Box :: new (P :: new (this , processor_data , options))) } fn parameter_descriptors (& self) -> Iterator { P :: parameter_descriptors () } }
};
}
