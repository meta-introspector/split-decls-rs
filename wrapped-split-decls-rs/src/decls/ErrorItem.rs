// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ErrorItem",
decl_type: "function",
source_file: "./src/line_counter.rs",
source_crate: ".",
deps: [],
uses: ["Clone", "Debug", "ErrorItem", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! ErrorItem {
    () => {
        # [derive (Debug , Clone)] pub struct ErrorItem { pub item_type : String , pub error_message : String , pub content_preview : String , }
    };
}

ErrorItem!();