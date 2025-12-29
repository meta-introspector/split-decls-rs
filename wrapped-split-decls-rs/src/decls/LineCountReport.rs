// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "LineCountReport",
decl_type: "function",
source_file: "./src/line_counter.rs",
source_crate: ".",
deps: ["SkippedItem", "ErrorItem"],
uses: ["SkippedItem", "LineCountReport", "ErrorItem", "Default", "Clone", "Vec", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SkippedItem!();
        ErrorItem!();
    };
}

macro_rules! LineCountReport {
    () => {
        deps!();
        # [derive (Debug , Clone , Default)] pub struct LineCountReport { pub input_lines : usize , pub output_lines : usize , pub skipped_items : Vec < SkippedItem > , pub processed_items : usize , pub error_items : Vec < ErrorItem > , }
    };
}

LineCountReport!();