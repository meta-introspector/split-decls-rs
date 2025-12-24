use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Differentiates between real files and common virtual files.
#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Decodable, Encodable)]
pub enum FileName {
    Real(RealFileName),
    /// Strings provided as `--cfg [cfgspec]`.
    CfgSpec(Hash64),
    /// Command line.
    Anon(Hash64),
    /// Hack in `src/librustc_ast/parse.rs`.
    MacroExpansion(Hash64),
    ProcMacroSourceCode(Hash64),
    /// Strings provided as crate attributes in the CLI.
    CliCrateAttr(Hash64),
    /// Custom sources for explicit parser calls from plugins and drivers.
    Custom(String),
    DocTest(PathBuf, isize),
    /// Post-substitution inline assembly from LLVM.
    InlineAsm(Hash64),
}
