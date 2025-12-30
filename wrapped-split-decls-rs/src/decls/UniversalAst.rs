// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "UniversalAst",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: [],
uses: ["Error", "UniversalAst", "TokenStream", "Result", "Item", "Universal", "AST"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! UniversalAst {
    () => {
        # [doc = " Universal AST execution context trait"] pub trait UniversalAst { type TokenStream ; type Item ; type Error ; fn parse_item (& self , input : Self :: TokenStream) -> Result < Self :: Item , Self :: Error > ; fn transform_item (& self , item : Self :: Item) -> Result < Self :: Item , Self :: Error > ; fn generate_code (& self , item : Self :: Item) -> Self :: TokenStream ; }
    };
}

UniversalAst!();