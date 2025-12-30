// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ProbeContext",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: ["AstNodeType"],
uses: ["PathBuf", "AstNodeType", "String", "ProbeContext", "Debug", "Vec", "Clone"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        AstNodeType!();
    };
}

macro_rules! ProbeContext {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct ProbeContext { pub file_path : PathBuf , pub node_name : String , pub node_type : AstNodeType , pub attributes : Vec < String > , pub visibility : String , pub complexity : f64 , }
    };
}

ProbeContext!();