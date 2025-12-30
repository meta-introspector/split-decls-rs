// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AstProbe",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: ["ProbeFilter", "AstNodeType", "ProbeAction"],
uses: ["ProbeFilter", "Clone", "Debug", "Deserialize", "Serialize", "AstProbe", "AstNodeType", "ProbeAction", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        ProbeFilter!();
        AstNodeType!();
        ProbeAction!();
    };
}

macro_rules! AstProbe {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AstProbe { pub name : String , pub node_type : AstNodeType , pub filter : ProbeFilter , pub action : ProbeAction , pub enabled : bool , pub priority : u32 , }
    };
}

AstProbe!();