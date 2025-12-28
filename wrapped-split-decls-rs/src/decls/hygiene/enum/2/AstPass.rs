use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " The kind of AST transform."] # [derive (Clone , Copy , Debug , PartialEq , Encodable , Decodable , HashStable_Generic)] pub enum AstPass { StdImports , TestHarness , ProcMacroHarness , }
}