// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_xtensaopts {
() => {
// Module: crate::spec::base::xtensa
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { os : "none" . into () , endian : Endian :: Little , c_int_width : 32 , linker_flavor : LinkerFlavor :: Gnu (Cc :: Yes , Lld :: No) , executables : true , panic_strategy : PanicStrategy :: Abort , relocation_model : RelocModel :: Static , emit_debug_gdb_scripts : false , atomic_cas : false , .. Default :: default () } }
};
}
