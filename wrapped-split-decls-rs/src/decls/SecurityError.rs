// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SecurityError",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: [],
uses: ["String", "SecurityError", "Debug"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! SecurityError {
    () => {
        # [derive (Debug)] pub struct SecurityError (pub String) ;
    };
}

SecurityError!();