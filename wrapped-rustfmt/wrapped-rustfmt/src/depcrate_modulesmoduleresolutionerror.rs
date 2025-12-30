// Generated macro for ModuleResolutionError (struct)
macro_rules! Depcrate_modulesModuleResolutionError {
() => {
// Module: crate::modules
// Provides: {"ModuleResolutionError"}
// Dependencies: {}
# [doc = " Represents errors while trying to resolve modules."] # [derive (Debug , Error)] # [error ("failed to resolve mod `{module}`: {kind}")] pub struct ModuleResolutionError { pub (crate) module : String , pub (crate) kind : ModuleResolutionErrorKind , }
};
}
