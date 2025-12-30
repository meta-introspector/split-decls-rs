// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SyscallAnalysisReport",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "Deserialize", "SyscallAnalysisReport", "Serialize"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! SyscallAnalysisReport {
    () => {
        # [derive (Debug , Serialize , Deserialize)] pub struct SyscallAnalysisReport { pub filesystem_calls : usize , pub process_calls : usize , pub environment_calls : usize , pub io_calls : usize , pub network_calls : usize , pub libc_calls : usize , pub time_calls : usize , pub total_syscalls : usize , }
    };
}

SyscallAnalysisReport!();