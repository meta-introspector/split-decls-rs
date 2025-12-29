// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "source_location",
decl_type: "function",
source_file: "./src/source_tracker.rs",
source_crate: ".",
deps: [],
uses: ["Attribute", "Usage"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! source_location {
    () => {
        # [doc = " Attribute macro to add source tracking information to a declaration"] # [doc = " Usage: #[source_location(file=\"src/lib.rs\", line=42, column=10)]"] # [macro_export] macro_rules ! source_location { (file = $ file : expr , line = $ line : expr , column = $ column : expr) => { } ; }
    };
}

source_location!();