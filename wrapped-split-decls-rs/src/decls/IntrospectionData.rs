// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "IntrospectionData",
decl_type: "function",
source_file: "./src/introspect_macro.rs",
source_crate: ".",
deps: [],
uses: ["TokenStream", "IntrospectionData", "Emergent", "CompileTime", "Debug", "Runtime", "String", "Clone"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! IntrospectionData {
    () => {
        # [derive (Debug , Clone)] pub enum IntrospectionData { CompileTime (TokenStream) , Runtime (String) , Emergent (String) , }
    };
}

IntrospectionData!();