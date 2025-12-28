macro_rules! deps {
    () => {
        FileMetadata!();
        ExtractionResult!();
        PublicSymbol!();
        DeclsVisitor!();
        ErrorSample!();
        RustcInfo!();
    };
}

macro_rules! extract_declarations_from_single_file {
    () => {
        deps!();
        pub async fn extract_declarations_from_single_file (file_path : & Path , _rustc_info : & RustcInfo , crate_name : & str , verbosity : u8 , warnings : & mut Vec < String > , _canonical_output_root : & Path ,) -> anyhow :: Result < ExtractionResult > { if verbosity >= 2 { warnings . push (format ! ("  [split-expanded-lib] extract_declarations_from_single_file: Processing file: {}" , file_path . display ())) ; } let mut file_content = tokio :: fs :: read_to_string (file_path) . await . context (format ! ("Failed to read file: {}" , file_path . display ())) ? ; let re_ansi = Regex :: new (r"\x1b\[[0-9;]*m") . unwrap () ; file_content = re_ansi . replace_all (& file_content , "") . to_string () ; let re_docs = Regex :: new (r"(?m)^\s*///.*$\n?") . unwrap () ; file_content = re_docs . replace_all (& file_content , "") . to_string () ; let mut file_metadata = FileMetadata :: default () ; let public_symbols : Vec < PublicSymbol > = Vec :: new () ; let errors : Vec < ErrorSample > = Vec :: new () ; let syntax_tree = syn :: parse_file (& file_content) . context (format ! ("Failed to parse file as Rust syntax tree: {}" , file_path . display ())) ? ; for item in & syntax_tree . items { match item { syn :: Item :: Use (item_use) => { file_metadata . global_uses . insert (item_use . to_token_stream () . to_string ()) ; } syn :: Item :: ExternCrate (item_extern_crate) => { file_metadata . extern_crates . insert (item_extern_crate . ident . to_string ()) ; } _ => { } } } let declarations ; let visitor_declarations_len ; { let mut visitor = DeclsVisitor :: new (file_path . to_path_buf () , crate_name . to_string () , verbosity , file_metadata . extern_crates . clone () , warnings ,) ; visitor . visit_file (& syntax_tree) ; declarations = visitor . declarations ; visitor_declarations_len = declarations . len () ; } if verbosity >= 2 { warnings . push (format ! ("  [split-expanded-lib] extract_declarations_from_single_file: Found {} declarations in {}" , visitor_declarations_len , file_path . display ())) ; } Ok (ExtractionResult { declarations , errors , file_metadata , public_symbols , }) }
    };
}

extract_declarations_from_single_file!();