// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "cached_fs_read_to_string",
decl_type: "function",
source_file: "./src/syscall_macros.rs",
source_crate: ".",
deps: [],
uses: [],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! cached_fs_read_to_string {
    () => {
        # [macro_export] macro_rules ! cached_fs_read_to_string { ($ path : expr) => { crate :: syscall :: cached_read_to_string ($ path) } ; }
    };
}

cached_fs_read_to_string!();