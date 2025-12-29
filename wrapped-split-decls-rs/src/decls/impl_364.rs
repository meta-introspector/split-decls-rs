// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_364",
decl_type: "function",
source_file: "./src/workflow_executor.rs",
source_crate: ".",
deps: ["Workflow", "WorkflowExecutor", "Stage"],
uses: ["Value", "HashMap", "SplitDeclsConfig", "Workflow", "Result", "WorkflowExecutor", "Option", "Ok", "Executing", "I/O", "Stage"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        Workflow!();
        WorkflowExecutor!();
        Stage!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        impl WorkflowExecutor { pub fn new (verbose : bool , dry_run : bool , global_config : SplitDeclsConfig) -> Self { WorkflowExecutor { verbose , dry_run , global_config , context : HashMap :: new () , } } pub fn execute (& mut self , workflow : & Workflow) -> Result < () > { if self . verbose { println ! ("Executing workflow: {}" , workflow . name) ; } for stage in & workflow . stages { self . execute_stage (stage) ? ; } Ok (()) } fn execute_stage (& mut self , stage : & Stage) -> Result < () > { if self . verbose { println ! ("  Executing stage: {}" , stage . name) ; } if self . verbose { println ! ("  ✅ Stage skipped - bootstrap only needs file I/O") ; } Ok (()) } pub fn get_context_value (& self , key : & str) -> Option < & toml :: Value > { self . context . get (key) } }
    };
}

impl_364!();