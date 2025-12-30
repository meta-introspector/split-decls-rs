// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_windows_uwp_msvcopts {
() => {
// Module: crate::spec::base::windows_uwp_msvc
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { let mut opts = base :: windows_msvc :: opts () ; opts . abi = "uwp" . into () ; opts . vendor = "uwp" . into () ; opts . add_pre_link_args (LinkerFlavor :: Msvc (Lld :: No) , & ["/APPCONTAINER" , "mincore.lib"]) ; opts }
};
}
