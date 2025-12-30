// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_lynxos178opts {
() => {
// Module: crate::spec::base::lynxos178
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { os : "lynxos178" . into () , dynamic_linking : false , families : cvs ! ["unix"] , position_independent_executables : false , static_position_independent_executables : false , relro_level : RelroLevel :: Full , has_thread_local : false , crt_static_respected : true , panic_strategy : PanicStrategy :: Abort , linker : Some (Cow :: Borrowed ("x86_64-lynx-lynxos178-gcc")) , no_default_libraries : false , eh_frame_header : false , max_atomic_width : Some (64) , supported_split_debuginfo : Cow :: Borrowed (& [SplitDebuginfo :: Packed , SplitDebuginfo :: Unpacked , SplitDebuginfo :: Off ,]) , relocation_model : RelocModel :: Static , stack_probes : StackProbeType :: Inline , .. Default :: default () } }
};
}
