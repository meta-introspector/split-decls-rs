// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "LineCountReport",
decl_type: "function",
source_file: "./src/line_counter.rs",
source_crate: ".",
deps: ["ErrorItem", "SkippedItem"],
uses: ["Vec", "ErrorItem", "Clone", "Default", "LineCountReport", "Debug", "SkippedItem"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        ErrorItem!();
        SkippedItem!();
    };
}

macro_rules! LineCountReport {
    () => {
        deps!();
        # [derive (Debug , Clone , Default)] pub struct LineCountReport { pub input_lines : usize , pub output_lines : usize , pub skipped_items : Vec < SkippedItem > , pub processed_items : usize , pub error_items : Vec < ErrorItem > , }
    };
}

LineCountReport!();