// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "interpret_syn_function",
decl_type: "function",
source_file: "./src/macro_interpreter.rs",
source_crate: ".",
deps: [],
uses: ["Wrapped", "Executed"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! interpret_syn_function {
    () => {
        # [macro_export] macro_rules ! interpret_syn_function { ($ rdf_state : expr , $ func_name : expr , $ wrap_path : expr) => { interpret_wrapped_decl ! ($ rdf_state , $ func_name , $ wrap_path , { let decl_content = std :: fs :: read_to_string ($ wrap_path) . unwrap_or_else (| _ | format ! ("// Wrapped declaration for {}" , $ func_name)) ; $ rdf_state . capture_data ("decl_size" , & decl_content . len () . to_string ()) ; $ rdf_state . capture_data ("decl_type" , "syn_function") ; format ! ("Executed wrapped: {}" , $ func_name) }) } ; }
    };
}

interpret_syn_function!();