// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "copy_declarations_to_output",
decl_type: "function",
source_file: "./src/eager_splitter.rs",
source_crate: ".",
deps: [],
uses: ["Copies", "Failed", "HashMap", "Ok", "Formatting", "The", "Error", "Result", "String", "Path", "Err"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! copy_declarations_to_output {
    () => {
        # [doc = " Copies extracted declarations to the specified output directory."] pub fn copy_declarations_to_output (crate_name : & str , declarations : & HashMap < String , String > , output_base_path : & Path ,) -> Result < () > { let crate_output_dir = output_base_path . join (crate_name) . join ("src") . join ("decls") ; std :: fs :: create_dir_all (& crate_output_dir) . context (format ! ("Failed to create output directory for declarations: {}" , crate_output_dir . display ())) ? ; for (decl_name , decl_tokens_str) in declarations { let file_path = crate_output_dir . join (format ! ("{}.rs" , decl_name)) ; let initial_content = add_generated_rust_header ! (decl_tokens_str . as_str () , file ! () , line ! ()) ; match format_rust_file (& initial_content , & file_path) { Ok (formatted_content) => { std :: fs :: write (& file_path , formatted_content) . context (format ! ("Failed to write formatted declaration to {}" , file_path . display ())) ? ; } , Err (e) => { let error_comment = format ! ("// !!! Formatting failed for this module. The following code is unformatted. !!!\n\
                    // !!! Error: {} !!!\n\n" , e) ; let content_with_error_comment = error_comment + & initial_content ; std :: fs :: write (& file_path , content_with_error_comment) . context (format ! ("Failed to write unformatted declaration with error comment to {}" , file_path . display ())) ? ; error ! ("\n<blip style='color:red'>Formatting error for '{} {}' (written to {})</blip>" , "declaration" , decl_name , file_path . display ()) ; } } } Ok (()) }
    };
}

copy_declarations_to_output!();