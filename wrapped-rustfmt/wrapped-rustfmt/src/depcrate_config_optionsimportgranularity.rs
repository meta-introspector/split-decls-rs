// Generated macro for ImportGranularity (enum)
macro_rules! Depcrate_config_optionsImportGranularity {
() => {
// Module: crate::config::options
// Provides: {"ImportGranularity"}
// Dependencies: {}
# [config_type] # [doc = " How to merge imports."] pub enum ImportGranularity { # [doc = " Do not merge imports."] Preserve , # [doc = " Use one `use` statement per crate."] Crate , # [doc = " Use one `use` statement per module."] Module , # [doc = " Use one `use` statement per imported item."] Item , # [doc = " Use one `use` statement including all items."] One , }
};
}
