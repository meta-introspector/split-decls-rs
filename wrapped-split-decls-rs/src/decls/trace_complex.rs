// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "trace_complex",
decl_type: "function",
source_file: "./src/bootstrap_tracer.rs",
source_crate: ".",
deps: [],
uses: [],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! trace_complex {
    () => {
        # [macro_export] macro_rules ! trace_complex { ($ tracer : expr , $ object : expr , complexity : $ level : expr , $ operation : expr , deps : [$ ($ dep : expr) ,*]) => { $ tracer . trace_complex_object ($ object , $ level , $ operation , vec ! [$ ($ dep . to_string ()) ,*]) } ; }
    };
}

trace_complex!();