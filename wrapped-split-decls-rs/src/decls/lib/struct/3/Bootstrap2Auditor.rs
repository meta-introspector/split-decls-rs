use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Bootstrap2: Audited reconstruction of split-decls-rs using generated declarations"] pub struct Bootstrap2Auditor { audit_log : Vec < String > , analyzer : Option < DeclarationAnalyzer > , emulator : EmulatedExecutor , workflow : WorkflowExecutor , interpreter : SynInterpreter , rdf_interpreter : RdfSynInterpreter , }
}