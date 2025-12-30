// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_teeosopts {
() => {
// Module: crate::spec::base::teeos
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { let lld_args = & ["-zmax-page-size=4096" , "-znow" , "-ztext" , "--execute-only"] ; let cc_args = & ["-Wl,-zmax-page-size=4096" , "-Wl,-znow" , "-Wl,-ztext" , "-mexecute-only"] ; let mut pre_link_args = TargetOptions :: link_args (LinkerFlavor :: Gnu (Cc :: No , Lld :: No) , lld_args) ; add_link_args (& mut pre_link_args , LinkerFlavor :: Gnu (Cc :: Yes , Lld :: No) , cc_args) ; TargetOptions { os : "teeos" . into () , vendor : "unknown" . into () , dynamic_linking : true , linker_flavor : LinkerFlavor :: Gnu (Cc :: Yes , Lld :: No) , has_rpath : false , has_thread_local : false , position_independent_executables : true , relro_level : RelroLevel :: Full , crt_static_respected : true , pre_link_args , panic_strategy : PanicStrategy :: Abort , .. Default :: default () } }
};
}
