// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SyscallInterceptor",
decl_type: "function",
source_file: "./src/syscall_oracle.rs",
source_crate: ".",
deps: ["TypeSafetyLevel", "SyscallWrapper"],
uses: ["HashMap", "String", "TypeSafetyLevel", "Clone", "Serialize", "Deserialize", "SyscallWrapper", "Debug", "SyscallInterceptor"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        TypeSafetyLevel!();
        SyscallWrapper!();
    };
}

macro_rules! SyscallInterceptor {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct SyscallInterceptor { pub syscall_mappings : HashMap < String , SyscallWrapper > , pub mock_mode : bool , pub dao_governance : bool , pub type_safety_level : TypeSafetyLevel , }
    };
}

SyscallInterceptor!();