// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "LockFileGuard",
decl_type: "function",
source_file: "./src/copy_dir_recursive.rs",
source_crate: ".",
deps: [],
uses: ["PathBuf", "LockFileGuard"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! LockFileGuard {
    () => {
        struct LockFileGuard { path : PathBuf , }
    };
}

LockFileGuard!();