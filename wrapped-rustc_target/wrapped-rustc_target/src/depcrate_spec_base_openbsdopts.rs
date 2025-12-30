// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_openbsdopts {
() => {
// Module: crate::spec::base::openbsd
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { os : "openbsd" . into () , dynamic_linking : true , families : cvs ! ["unix"] , has_rpath : true , abi_return_struct_as_int : true , position_independent_executables : true , frame_pointer : FramePointer :: Always , relro_level : RelroLevel :: Full , default_dwarf_version : 2 , tls_model : TlsModel :: Emulated , .. Default :: default () } }
};
}
