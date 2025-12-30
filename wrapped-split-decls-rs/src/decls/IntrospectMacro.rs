// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "IntrospectMacro",
decl_type: "function",
source_file: "./src/introspect_macro.rs",
source_crate: ".",
deps: ["InjectionPoint", "BottLevel"],
uses: ["Bott", "InjectionPoint", "Introspection", "IntrospectMacro", "Vec", "BottLevel"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        InjectionPoint!();
        BottLevel!();
    };
}

macro_rules! IntrospectMacro {
    () => {
        deps!();
        # [doc = " Introspection macro that injects self-awareness at any Bott level"] pub struct IntrospectMacro { current_level : BottLevel , injection_points : Vec < InjectionPoint > , }
    };
}

IntrospectMacro!();