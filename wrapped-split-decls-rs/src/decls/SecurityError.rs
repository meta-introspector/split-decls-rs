// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SecurityError",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: [],
uses: ["String", "Debug", "SecurityError"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! SecurityError {
    () => {
        # [derive (Debug)] pub struct SecurityError (pub String) ;
    };
}

SecurityError!();