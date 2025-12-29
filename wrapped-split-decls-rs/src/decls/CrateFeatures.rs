// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CrateFeatures",
decl_type: "function",
source_file: "./src/rustc_eigenmatrix.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "String", "Clone", "Vec", "CrateFeatures"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! CrateFeatures {
    () => {
        # [derive (Debug , Clone)] pub struct CrateFeatures { pub name : String , pub path : String , pub functions : usize , pub structs : usize , pub enums : usize , pub macros : usize , pub traits : usize , pub impls : usize , pub loc : usize , pub dependencies : Vec < String > , }
    };
}

CrateFeatures!();