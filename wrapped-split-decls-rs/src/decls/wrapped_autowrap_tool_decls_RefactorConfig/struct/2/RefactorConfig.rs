use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Deserialize)] pub struct RefactorConfig { pub refactor : RefactorMeta , # [serde (default)] pub splits : Vec < SplitRule > , pub export : ExportConfig , # [serde (default)] pub decl_refactoring : Option < DeclRefactoringConfig > , }