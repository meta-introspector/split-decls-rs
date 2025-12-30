// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "TraitDecouplingTemplate",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: ["ImplAdapter", "SyscallTrait", "WrapperMacro"],
uses: ["Serialize", "Debug", "Vec", "ImplAdapter", "Deserialize", "SyscallTrait", "TraitDecouplingTemplate", "WrapperMacro"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        ImplAdapter!();
        SyscallTrait!();
        WrapperMacro!();
    };
}

macro_rules! TraitDecouplingTemplate {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] pub struct TraitDecouplingTemplate { pub syscall_traits : Vec < SyscallTrait > , pub wrapper_macros : Vec < WrapperMacro > , pub implementation_adapters : Vec < ImplAdapter > , }
    };
}

TraitDecouplingTemplate!();