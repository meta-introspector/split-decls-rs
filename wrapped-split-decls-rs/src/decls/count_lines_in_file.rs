// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "count_lines_in_file",
decl_type: "function",
source_file: "./src/line_counter.rs",
source_crate: ".",
deps: [],
uses: ["Ok", "Path", "Result"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! count_lines_in_file {
    () => {
        pub fn count_lines_in_file (path : & Path) -> Result < usize > { let content = std :: fs :: read_to_string (path) ? ; Ok (content . lines () . count ()) }
    };
}

count_lines_in_file!();