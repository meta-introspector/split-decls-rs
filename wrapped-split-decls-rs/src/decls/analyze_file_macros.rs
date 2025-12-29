// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "analyze_file_macros",
decl_type: "function",
source_file: "./src/macro_analyzer_parts/file_analyzer.rs",
source_crate: ".",
deps: ["Term", "TermCollector"],
uses: ["Item", "Vec", "Rust", "Result", "Term", "Fn", "HashMap", "String", "Public", "TermCollector", "Ok", "Path", "Visibility", "Failed"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        Term!();
        TermCollector!();
    };
}

macro_rules! analyze_file_macros {
    () => {
        deps!();
        pub fn analyze_file_macros (file_path : & Path) -> Result < HashMap < String , Vec < Term > > > { let mut content = fs :: read_to_string (file_path) . with_context (| | format ! ("Failed to read file: {}" , file_path . display ())) ? ; if content . trim_end () . ends_with (". sig") { let trimmed_len = content . trim_end () . len () ; content . truncate (trimmed_len - ". sig" . len ()) ; } content = content . replace ("# [" , "#[") ; content = content . replace (" } " , "}\n") ; content = content . replace (" ] " , "]\n") ; let ast = parse_file (& content) . with_context (| | format ! ("Failed to parse Rust file: {}" , file_path . display ())) ? ; let mut macro_terms : HashMap < String , Vec < Term > > = HashMap :: new () ; for item in ast . items { if let syn :: Item :: Fn (func) = item { let is_pub = matches ! (func . vis , syn :: Visibility :: Public (_)) ; if func . sig . ident . to_string () . ends_with ("_impl") && is_pub { let mut collector = TermCollector :: default () ; collector . visit_item_fn (& func) ; macro_terms . insert (func . sig . ident . to_string () , collector . terms) ; } } } Ok (macro_terms) }
    };
}

analyze_file_macros!();