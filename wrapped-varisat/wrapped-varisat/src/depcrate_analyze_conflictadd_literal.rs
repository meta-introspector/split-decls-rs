// Generated macro for add_literal (function)
macro_rules! Depcrate_analyze_conflictadd_literal {
() => {
// Module: crate::analyze_conflict
// Provides: {"add_literal"}
// Dependencies: {}
# [doc = " Add a literal to the current clause."] fn add_literal (mut ctx : partial ! (Context , mut AnalyzeConflictP , mut VsidsP , ImplGraphP , TrailP) , lit : Lit ,) { let (analyze , mut ctx) = ctx . split_part_mut (AnalyzeConflictP) ; let lit_level = ctx . part (ImplGraphP) . level (lit . var ()) ; if lit_level > 0 && ! analyze . var_flags [lit . index ()] { ctx . part_mut (VsidsP) . bump (lit . var ()) ; analyze . var_flags [lit . index ()] = true ; if lit_level == ctx . part (TrailP) . current_level () { analyze . current_level_count += 1 ; } else { analyze . clause . push (lit) ; analyze . to_clean . push (lit . var ()) ; } } }
};
}
