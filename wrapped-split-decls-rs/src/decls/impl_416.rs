// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_416",
decl_type: "function",
source_file: "./src/copy_dir_recursive.rs",
source_crate: ".",
deps: ["LockFileGuard"],
uses: ["Warning", "LockFileGuard", "Failed", "Drop", "Err"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        LockFileGuard!();
    };
}

macro_rules! impl_416 {
    () => {
        deps!();
        impl Drop for LockFileGuard { fn drop (& mut self) { if let Err (e) = fs :: remove_file (& self . path) { eprintln ! ("Warning: Failed to remove lock file at {}: {}" , self . path . display () , e) ; } } }
    };
}

impl_416!();