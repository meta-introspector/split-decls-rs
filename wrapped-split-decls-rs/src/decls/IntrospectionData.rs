// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "IntrospectionData",
decl_type: "function",
source_file: "./src/introspect_macro.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "CompileTime", "Emergent", "String", "IntrospectionData", "Runtime", "TokenStream", "Clone"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! IntrospectionData {
    () => {
        # [derive (Debug , Clone)] pub enum IntrospectionData { CompileTime (TokenStream) , Runtime (String) , Emergent (String) , }
    };
}

IntrospectionData!();