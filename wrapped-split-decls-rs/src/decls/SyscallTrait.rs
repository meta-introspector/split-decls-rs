// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SyscallTrait",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: ["TraitMethod", "SafetyLevel", "SyscallCategory"],
uses: ["String", "TraitMethod", "Serialize", "SafetyLevel", "Debug", "Deserialize", "SyscallTrait", "Vec", "SyscallCategory"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        TraitMethod!();
        SafetyLevel!();
        SyscallCategory!();
    };
}

macro_rules! SyscallTrait {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] pub struct SyscallTrait { pub name : String , pub category : SyscallCategory , pub methods : Vec < TraitMethod > , pub safety_level : SafetyLevel , }
    };
}

SyscallTrait!();