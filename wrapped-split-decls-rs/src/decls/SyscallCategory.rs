// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SyscallCategory",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: [],
uses: ["Time", "Clone", "Debug", "IO", "FileSystem", "Process", "Serialize", "Environment", "Network", "Deserialize", "SyscallCategory", "Memory"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! SyscallCategory {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum SyscallCategory { FileSystem , Process , Environment , IO , Network , Memory , Time , }
    };
}

SyscallCategory!();