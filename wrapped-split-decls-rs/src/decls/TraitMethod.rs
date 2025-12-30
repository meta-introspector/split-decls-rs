// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "TraitMethod",
decl_type: "function",
source_file: "./src/syscall_decoupling_template.rs",
source_crate: ".",
deps: [],
uses: ["Serialize", "Deserialize", "TraitMethod", "String", "Debug", "Vec"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! TraitMethod {
    () => {
        # [derive (Debug , Serialize , Deserialize)] pub struct TraitMethod { pub name : String , pub inputs : Vec < String > , pub output : String , pub original_syscall : String , }
    };
}

TraitMethod!();