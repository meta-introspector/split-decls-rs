// Generated macro for IdentifierUncommonCodepoints (struct)
macro_rules! Depcrate_lintsIdentifierUncommonCodepoints {
() => {
// Module: crate::lints
// Provides: {"IdentifierUncommonCodepoints"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_identifier_uncommon_codepoints)] # [note] pub (crate) struct IdentifierUncommonCodepoints { pub codepoints : Vec < char > , pub codepoints_len : usize , pub identifier_type : & 'static str , }
};
}
