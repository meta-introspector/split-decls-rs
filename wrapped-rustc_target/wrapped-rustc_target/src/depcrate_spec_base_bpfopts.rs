// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_bpfopts {
() => {
// Module: crate::spec::base::bpf
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts (endian : Endian) -> TargetOptions { TargetOptions { allow_asm : true , endian , linker_flavor : LinkerFlavor :: Bpf , atomic_cas : false , dynamic_linking : true , no_builtins : true , panic_strategy : PanicStrategy :: Abort , position_independent_executables : true , merge_functions : MergeFunctions :: Disabled , obj_is_bitcode : true , requires_lto : false , singlethread : true , min_atomic_width : Some (64) , max_atomic_width : Some (64) , .. Default :: default () } }
};
}
