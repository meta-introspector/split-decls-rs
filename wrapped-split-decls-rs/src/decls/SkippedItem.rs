// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SkippedItem",
decl_type: "function",
source_file: "./src/line_counter.rs",
source_crate: ".",
deps: [],
uses: ["String", "Debug", "Clone", "SkippedItem"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! SkippedItem {
    () => {
        # [derive (Debug , Clone)] pub struct SkippedItem { pub item_type : String , pub reason : String , pub content_preview : String , }
    };
}

SkippedItem!();