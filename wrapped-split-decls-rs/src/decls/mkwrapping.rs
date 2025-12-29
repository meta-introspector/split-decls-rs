// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "mkwrapping",
decl_type: "function",
source_file: "./src/config_macros.rs",
source_crate: ".",
deps: [],
uses: ["GLOBAL_CONFIG"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! mkwrapping {
    () => {
        # [macro_export] macro_rules ! mkwrapping { () => { crate :: config_macros :: GLOBAL_CONFIG . lock () . unwrap () . wrapping . clone () } ; }
    };
}

mkwrapping!();