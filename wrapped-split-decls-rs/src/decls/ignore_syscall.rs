// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ignore_syscall",
decl_type: "function",
source_file: "./src/syscall_macros.rs",
source_crate: ".",
deps: [],
uses: [],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! ignore_syscall {
    () => {
        # [macro_export] macro_rules ! ignore_syscall { ($ ($ tt : tt) *) => { $ ($ tt) * } ; }
    };
}

ignore_syscall!();