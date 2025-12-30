// Generated macro for compute_glue (function)
macro_rules! Depcrate_gluecompute_glue {
() => {
// Module: crate::glue
// Provides: {"compute_glue"}
// Dependencies: {}
# [doc = " Compute the glue level of a clause."] pub fn compute_glue (mut ctx : partial ! (Context , mut TmpFlagsP , ImplGraphP) , lits : & [Lit]) -> usize { let (tmp_flags , ctx) = ctx . split_part_mut (TmpFlagsP) ; let impl_graph = ctx . part (ImplGraphP) ; let flags = & mut tmp_flags . flags ; let mut glue = 0 ; for & lit in lits { let level = impl_graph . level (lit . var ()) ; let flag = & mut flags [level] ; if ! * flag { * flag = true ; glue += 1 } } for & lit in lits { let level = impl_graph . level (lit . var ()) ; flags [level] = false ; } glue }
};
}
