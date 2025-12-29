// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "interpret_wrapped_decl",
decl_type: "function",
source_file: "./src/macro_interpreter.rs",
source_crate: ".",
deps: [],
uses: [],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! interpret_wrapped_decl {
    () => {
        # [macro_export] macro_rules ! interpret_wrapped_decl { ($ rdf_state : expr , $ func_name : expr , $ wrap_path : expr , $ body : block) => { { $ rdf_state . enter_function ($ func_name) ; $ rdf_state . capture_data ("wrap_path" , $ wrap_path) ; let result = $ body ; $ rdf_state . exit_function ($ func_name) ; result } } ; }
    };
}

interpret_wrapped_decl!();