// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_thumbopts {
() => {
// Module: crate::spec::base::thumb
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { linker_flavor : LinkerFlavor :: Gnu (Cc :: No , Lld :: Yes) , linker : Some ("rust-lld" . into ()) , panic_strategy : PanicStrategy :: Abort , relocation_model : RelocModel :: Static , emit_debug_gdb_scripts : false , frame_pointer : FramePointer :: Always , c_enum_min_bits : Some (8) , .. Default :: default () } }
};
}
