// Generated macro for link_env_remove (function)
macro_rules! Depcrate_spec_base_applelink_env_remove {
() => {
// Module: crate::spec::base::apple
// Provides: {"link_env_remove"}
// Dependencies: {}
fn link_env_remove (os : & 'static str) -> StaticCow < [StaticCow < str >] > { if os == "macos" { cvs ! ["IPHONEOS_DEPLOYMENT_TARGET" , "TVOS_DEPLOYMENT_TARGET" , "XROS_DEPLOYMENT_TARGET"] } else { cvs ! ["MACOSX_DEPLOYMENT_TARGET"] } }
};
}
