// Generated macro for GroupImportsTactic (enum)
macro_rules! Depcrate_config_optionsGroupImportsTactic {
() => {
// Module: crate::config::options
// Provides: {"GroupImportsTactic"}
// Dependencies: {}
# [config_type] # [doc = " Configuration for import groups, i.e. sets of imports separated by newlines."] pub enum GroupImportsTactic { # [doc = " Keep groups as they are."] Preserve , # [doc = " Discard existing groups, and create new groups for"] # [doc = "  1. `std` / `core` / `alloc` imports"] # [doc = "  2. other imports"] # [doc = "  3. `self` / `crate` / `super` imports"] StdExternalCrate , # [doc = " Discard existing groups, and create a single group for everything"] One , }
};
}
