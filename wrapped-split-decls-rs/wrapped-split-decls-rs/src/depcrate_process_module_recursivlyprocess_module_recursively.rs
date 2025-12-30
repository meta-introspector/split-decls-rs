// Generated macro for process_module_recursively (function)
macro_rules! Depcrate_process_module_recursivlyprocess_module_recursively {
() => {
// Module: crate::process_module_recursivly
// Provides: {"process_module_recursively"}
// Dependencies: {}
pub fn process_module_recursively (paths : & CratePaths , config : & SplitDeclsConfig , mod_name : & str , parent_path : & str , collected_module_names : & mut Vec < Ident > , item_count : & mut usize , common_uses : & TokenStream , dry_run : bool , module_not_found_errors : & mut Vec < ModuleNotFoundReport > ,) -> Result < () > { process_module_recursively_with_depth (paths , config , mod_name , parent_path , collected_module_names , item_count , common_uses , dry_run , module_not_found_errors , 0) }
};
}
