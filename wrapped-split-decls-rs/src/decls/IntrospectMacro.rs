// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "IntrospectMacro",
decl_type: "function",
source_file: "./src/introspect_macro.rs",
source_crate: ".",
deps: ["BottLevel", "InjectionPoint"],
uses: ["Vec", "Introspection", "Bott", "BottLevel", "InjectionPoint", "IntrospectMacro"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        BottLevel!();
        InjectionPoint!();
    };
}

macro_rules! IntrospectMacro {
    () => {
        deps!();
        # [doc = " Introspection macro that injects self-awareness at any Bott level"] pub struct IntrospectMacro { current_level : BottLevel , injection_points : Vec < InjectionPoint > , }
    };
}

IntrospectMacro!();