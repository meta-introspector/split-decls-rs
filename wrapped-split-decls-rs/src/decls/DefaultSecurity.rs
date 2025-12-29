// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "DefaultSecurity",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: [],
uses: ["Default", "DefaultSecurity"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! DefaultSecurity {
    () => {
        # [doc = " Default security implementation (permissive for development)"] pub struct DefaultSecurity ;
    };
}

DefaultSecurity!();