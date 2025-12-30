// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "LockFileGuard",
decl_type: "function",
source_file: "./src/copy_dir_recursive.rs",
source_crate: ".",
deps: [],
uses: ["LockFileGuard", "PathBuf"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! LockFileGuard {
    () => {
        struct LockFileGuard { path : PathBuf , }
    };
}

LockFileGuard!();