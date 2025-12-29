// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SyscallOracle",
decl_type: "function",
source_file: "./src/syscall_traits.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "SyscallOracle", "Core"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! SyscallOracle {
    () => {
        # [doc = " Core trait for syscall auditing and safety"] pub trait SyscallOracle { fn audit_call (function_name : & str , syscall_type : & str) ; fn pre_call_hook (syscall_type : & str) ; fn post_call_hook (syscall_type : & str , result : & dyn std :: fmt :: Debug) ; }
    };
}

SyscallOracle!();