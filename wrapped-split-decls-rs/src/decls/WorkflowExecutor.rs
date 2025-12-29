// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "WorkflowExecutor",
decl_type: "function",
source_file: "./src/workflow_executor.rs",
source_crate: ".",
deps: [],
uses: ["HashMap", "WorkflowExecutor", "SplitDeclsConfig", "String", "Value"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! WorkflowExecutor {
    () => {
        pub struct WorkflowExecutor { verbose : bool , dry_run : bool , global_config : SplitDeclsConfig , context : HashMap < String , toml :: Value > , }
    };
}

WorkflowExecutor!();