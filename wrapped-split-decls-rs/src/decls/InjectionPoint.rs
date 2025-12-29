// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "InjectionPoint",
decl_type: "function",
source_file: "./src/introspect_macro.rs",
source_crate: ".",
deps: ["IntrospectionData", "BottLevel"],
uses: ["String", "IntrospectionData", "Debug", "BottLevel", "Clone", "InjectionPoint"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        IntrospectionData!();
        BottLevel!();
    };
}

macro_rules! InjectionPoint {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct InjectionPoint { pub location : String , pub bott_level : BottLevel , pub introspection_data : IntrospectionData , }
    };
}

InjectionPoint!();