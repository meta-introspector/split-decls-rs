// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "span_to_location",
decl_type: "function",
source_file: "./src/source_tracker.rs",
source_crate: ".",
deps: ["SourceLocation"],
uses: ["LineColumn", "Span", "SourceLocation", "Extract"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SourceLocation!();
    };
}

macro_rules! span_to_location {
    () => {
        deps!();
        # [doc = " Extract source location from a span"] pub fn span_to_location (span : Span) -> SourceLocation { let line_col : LineColumn = span . start () ; SourceLocation { file : "unknown" . to_string () , line : line_col . line , column : line_col . column , } }
    };
}

span_to_location!();