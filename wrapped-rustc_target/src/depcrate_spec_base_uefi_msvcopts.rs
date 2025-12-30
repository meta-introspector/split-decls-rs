// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_uefi_msvcopts {
() => {
// Module: crate::spec::base::uefi_msvc
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { let mut base = base :: msvc :: opts () ; base . add_pre_link_args (LinkerFlavor :: Msvc (Lld :: No) , & ["/entry:efi_main" , "/subsystem:efi_application" ,] ,) ; TargetOptions { os : "uefi" . into () , linker_flavor : LinkerFlavor :: Msvc (Lld :: Yes) , disable_redzone : true , exe_suffix : ".efi" . into () , allows_weak_linkage : false , panic_strategy : PanicStrategy :: Abort , stack_probes : StackProbeType :: Call , singlethread : true , linker : Some ("rust-lld" . into ()) , entry_name : "efi_main" . into () , .. base } }
};
}
