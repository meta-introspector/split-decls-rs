// Generated macro for NonUpperCaseGlobal (struct)
macro_rules! Depcrate_lintsNonUpperCaseGlobal {
() => {
// Module: crate::lints
// Provides: {"NonUpperCaseGlobal"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_non_upper_case_global)] pub (crate) struct NonUpperCaseGlobal < 'a > { pub sort : & 'a str , pub name : & 'a str , # [subdiagnostic] pub sub : NonUpperCaseGlobalSub , # [subdiagnostic] pub usages : Vec < NonUpperCaseGlobalSubTool > , }
};
}
