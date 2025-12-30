// Generated macro for impl_79 (impl)
macro_rules! Depcrate_scannerimpl_79 {
() => {
// Module: crate::scanner
// Provides: {"impl_79"}
// Dependencies: {}
impl SkipTabs { # [doc = " Whether tabs were found while skipping whitespace."] # [doc = ""] # [doc = " This function must be called after a call to `skip_ws_to_eol`."] fn found_tabs (self) -> bool { matches ! (self , SkipTabs :: Result (true , _)) } # [doc = " Whether a valid YAML whitespace has been found in skipped-over content."] # [doc = ""] # [doc = " This function must be called after a call to `skip_ws_to_eol`."] fn has_valid_yaml_ws (self) -> bool { matches ! (self , SkipTabs :: Result (_ , true)) } }
};
}
