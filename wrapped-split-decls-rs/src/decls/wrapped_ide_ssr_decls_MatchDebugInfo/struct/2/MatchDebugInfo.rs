use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct MatchDebugInfo { node : SyntaxNode , # [doc = " Our search pattern parsed as an expression or item, etc"] pattern : SyntaxNode , matched : Result < Match , MatchFailureReason > , }