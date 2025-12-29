// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "count_lines_in_string",
decl_type: "function",
source_file: "./src/line_counter.rs",
source_crate: ".",
deps: [],
uses: [],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! count_lines_in_string {
    () => {
        pub fn count_lines_in_string (content : & str) -> usize { content . lines () . count () }
    };
}

count_lines_in_string!();