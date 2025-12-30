// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "generate_trace_header",
decl_type: "function",
source_file: "./src/trace_header.rs",
source_crate: ".",
deps: [],
uses: ["Generates", "Option", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! generate_trace_header {
    () => {
        # [doc = " Generates a trace header for any generated file"] pub fn generate_trace_header (operation : & str , input_file : Option < & str > , processor_function : & str , source_location : & str ,) -> String { generate_trace_header_with_comment_style (operation , input_file , processor_function , source_location , "//") }
    };
}

generate_trace_header!();