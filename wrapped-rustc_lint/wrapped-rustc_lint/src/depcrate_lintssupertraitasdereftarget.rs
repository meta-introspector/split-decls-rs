// Generated macro for SupertraitAsDerefTarget (struct)
macro_rules! Depcrate_lintsSupertraitAsDerefTarget {
() => {
// Module: crate::lints
// Provides: {"SupertraitAsDerefTarget"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_supertrait_as_deref_target)] pub (crate) struct SupertraitAsDerefTarget < 'a > { pub self_ty : Ty < 'a > , pub supertrait_principal : PolyExistentialTraitRef < 'a > , pub target_principal : PolyExistentialTraitRef < 'a > , # [label] pub label : Span , # [subdiagnostic] pub label2 : Option < SupertraitAsDerefTargetLabel > , }
};
}
