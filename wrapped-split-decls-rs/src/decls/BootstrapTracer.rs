// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "BootstrapTracer",
decl_type: "function",
source_file: "./src/bootstrap_tracer.rs",
source_crate: ".",
deps: ["BootstrapTrace"],
uses: ["String", "BootstrapTrace", "Option", "BootstrapTracer"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        BootstrapTrace!();
    };
}

macro_rules! BootstrapTracer {
    () => {
        deps!();
        pub struct BootstrapTracer { pub trace : BootstrapTrace , pub current_state : Option < String > , }
    };
}

BootstrapTracer!();