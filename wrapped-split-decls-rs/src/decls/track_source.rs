// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "track_source",
decl_type: "function",
source_file: "./src/source_tracker.rs",
source_crate: ".",
deps: [],
uses: ["Macro", "Usage"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! track_source {
    () => {
        # [doc = " Macro to track the source location of a code element"] # [doc = " Usage: track_source!(expression, file!(), line!(), column!())"] # [macro_export] macro_rules ! track_source { ($ expr : expr , $ file : expr , $ line : expr , $ column : expr) => { { $ expr } } ; }
    };
}

track_source!();