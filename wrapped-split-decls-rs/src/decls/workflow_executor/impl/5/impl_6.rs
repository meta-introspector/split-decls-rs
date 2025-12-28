use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl WorkflowExecutor { pub fn new (verbose : bool , dry_run : bool , global_config : SplitDeclsConfig) -> Self { WorkflowExecutor { verbose , dry_run , global_config , context : HashMap :: new () , } } pub fn execute (& mut self , workflow : & Workflow) -> Result < () > { if self . verbose { println ! ("Executing workflow: {}" , workflow . name) ; } for stage in & workflow . stages { self . execute_stage (stage) ? ; } Ok (()) } fn execute_stage (& mut self , stage : & Stage) -> Result < () > { if self . verbose { println ! ("  Executing stage: {}" , stage . name) ; } if self . verbose { println ! ("  ✅ Stage skipped - bootstrap only needs file I/O") ; } Ok (()) } pub fn get_context_value (& self , key : & str) -> Option < & toml :: Value > { self . context . get (key) } }
}