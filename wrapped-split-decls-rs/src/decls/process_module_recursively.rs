// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "process_module_recursively",
decl_type: "function",
source_file: "./src/process_module_recursivly.rs",
source_crate: ".",
deps: ["ModuleNotFoundReport", "CratePaths"],
uses: ["Result", "SplitDeclsConfig", "TokenStream", "ModuleNotFoundReport", "CratePaths", "Vec", "Ident"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        ModuleNotFoundReport!();
        CratePaths!();
    };
}

macro_rules! process_module_recursively {
    () => {
        deps!();
        pub fn process_module_recursively (paths : & CratePaths , config : & SplitDeclsConfig , mod_name : & str , parent_path : & str , collected_module_names : & mut Vec < Ident > , item_count : & mut usize , common_uses : & TokenStream , dry_run : bool , module_not_found_errors : & mut Vec < ModuleNotFoundReport > ,) -> Result < () > { process_module_recursively_with_depth (paths , config , mod_name , parent_path , collected_module_names , item_count , common_uses , dry_run , module_not_found_errors , 0) }
    };
}

process_module_recursively!();