// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "InjectionPoint",
decl_type: "function",
source_file: "./src/introspect_macro.rs",
source_crate: ".",
deps: ["IntrospectionData", "BottLevel"],
uses: ["IntrospectionData", "String", "Clone", "InjectionPoint", "Debug", "BottLevel"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
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