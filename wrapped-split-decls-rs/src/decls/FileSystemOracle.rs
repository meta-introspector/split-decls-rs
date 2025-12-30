// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "FileSystemOracle",
decl_type: "function",
source_file: "./src/syscall_traits.rs",
source_crate: ".",
deps: [],
uses: ["FileSystemOracle", "Filesystem", "Result", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! FileSystemOracle {
    () => {
        # [doc = " Filesystem operations oracle"] pub trait FileSystemOracle { fn audit_read () -> Result < () , String > ; fn audit_write () -> Result < () , String > ; fn check_path_safety (path : & str) -> bool ; }
    };
}

FileSystemOracle!();