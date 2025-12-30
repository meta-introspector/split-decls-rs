// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CrateFeatures",
decl_type: "function",
source_file: "./src/rustc_eigenmatrix.rs",
source_crate: ".",
deps: [],
uses: ["String", "CrateFeatures", "Clone", "Debug", "Vec"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! CrateFeatures {
    () => {
        # [derive (Debug , Clone)] pub struct CrateFeatures { pub name : String , pub path : String , pub functions : usize , pub structs : usize , pub enums : usize , pub macros : usize , pub traits : usize , pub impls : usize , pub loc : usize , pub dependencies : Vec < String > , }
    };
}

CrateFeatures!();