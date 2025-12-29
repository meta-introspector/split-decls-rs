// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "TraitDecouplingTemplate",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: ["ImplAdapter", "WrapperMacro", "SyscallTrait"],
uses: ["ImplAdapter", "Serialize", "WrapperMacro", "Debug", "TraitDecouplingTemplate", "Vec", "Deserialize", "SyscallTrait"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        ImplAdapter!();
        WrapperMacro!();
        SyscallTrait!();
    };
}

macro_rules! TraitDecouplingTemplate {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] pub struct TraitDecouplingTemplate { pub syscall_traits : Vec < SyscallTrait > , pub wrapper_macros : Vec < WrapperMacro > , pub implementation_adapters : Vec < ImplAdapter > , }
    };
}

TraitDecouplingTemplate!();