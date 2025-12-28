use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl AstPass { pub fn descr (self) -> & 'static str { match self { AstPass :: StdImports => "standard library imports" , AstPass :: TestHarness => "test harness" , AstPass :: ProcMacroHarness => "proc macro harness" , } } }