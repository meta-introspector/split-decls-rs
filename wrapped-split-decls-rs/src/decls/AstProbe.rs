// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AstProbe",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: ["ProbeAction", "AstNodeType", "ProbeFilter"],
uses: ["ProbeAction", "Debug", "Deserialize", "AstProbe", "String", "Serialize", "Clone", "AstNodeType", "ProbeFilter"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        ProbeAction!();
        AstNodeType!();
        ProbeFilter!();
    };
}

macro_rules! AstProbe {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AstProbe { pub name : String , pub node_type : AstNodeType , pub filter : ProbeFilter , pub action : ProbeAction , pub enabled : bool , pub priority : u32 , }
    };
}

AstProbe!();