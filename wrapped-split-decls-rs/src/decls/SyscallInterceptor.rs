// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SyscallInterceptor",
decl_type: "function",
source_file: "./src/syscall_oracle.rs",
source_crate: ".",
deps: ["SyscallWrapper", "TypeSafetyLevel"],
uses: ["Clone", "String", "HashMap", "Debug", "SyscallInterceptor", "SyscallWrapper", "TypeSafetyLevel", "Serialize", "Deserialize"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SyscallWrapper!();
        TypeSafetyLevel!();
    };
}

macro_rules! SyscallInterceptor {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct SyscallInterceptor { pub syscall_mappings : HashMap < String , SyscallWrapper > , pub mock_mode : bool , pub dao_governance : bool , pub type_safety_level : TypeSafetyLevel , }
    };
}

SyscallInterceptor!();