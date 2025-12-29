// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SyscallAstTransformer",
decl_type: "function",
source_file: "./src/syscall_oracle.rs",
source_crate: ".",
deps: ["SyscallInterceptor"],
uses: ["String", "HashMap", "SyscallInterceptor", "SyscallAstTransformer"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SyscallInterceptor!();
    };
}

macro_rules! SyscallAstTransformer {
    () => {
        deps!();
        pub struct SyscallAstTransformer { interceptor : SyscallInterceptor , transformations : HashMap < String , usize > , }
    };
}

SyscallAstTransformer!();