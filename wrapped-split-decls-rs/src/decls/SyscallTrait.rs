// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SyscallTrait",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: ["SyscallCategory", "TraitMethod", "SafetyLevel"],
uses: ["Deserialize", "SyscallCategory", "SyscallTrait", "Serialize", "Debug", "String", "TraitMethod", "SafetyLevel", "Vec"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SyscallCategory!();
        TraitMethod!();
        SafetyLevel!();
    };
}

macro_rules! SyscallTrait {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] pub struct SyscallTrait { pub name : String , pub category : SyscallCategory , pub methods : Vec < TraitMethod > , pub safety_level : SafetyLevel , }
    };
}

SyscallTrait!();