// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "trace_step",
decl_type: "function",
source_file: "./src/bootstrap_tracer.rs",
source_crate: ".",
deps: [],
uses: [],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! trace_step {
    () => {
        # [macro_export] macro_rules ! trace_step { ($ tracer : expr , $ step : expr , inputs : [$ ($ input : expr) ,*] , outputs : [$ ($ output : expr) ,*]) => { $ tracer . trace_step ($ step , vec ! [$ ($ input . to_string ()) ,*] , vec ! [$ ($ output . to_string ()) ,*]) } ; }
    };
}

trace_step!();