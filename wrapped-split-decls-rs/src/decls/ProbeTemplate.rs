// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ProbeTemplate",
decl_type: "function",
source_file: "./src/sparql_probe_bridge.rs",
source_crate: ".",
deps: ["AstNodeType"],
uses: ["Option", "ProbeTemplate", "AstNodeType", "Debug", "String", "Deserialize", "Serialize"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        AstNodeType!();
    };
}

macro_rules! ProbeTemplate {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] pub struct ProbeTemplate { pub node_type : AstNodeType , pub action_template : String , pub wrapper_function : Option < String > , pub priority : u32 , }
    };
}

ProbeTemplate!();