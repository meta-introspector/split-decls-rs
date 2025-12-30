// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ProbeFilter",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: [],
uses: ["Serialize", "Debug", "Deserialize", "String", "Option", "ProbeFilter", "Vec", "Clone"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! ProbeFilter {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProbeFilter { pub name_pattern : Option < String > , pub visibility : Option < String > , pub attributes : Vec < String > , pub contains_text : Option < String > , pub complexity_threshold : Option < f64 > , pub layer : Option < String > , }
    };
}

ProbeFilter!();