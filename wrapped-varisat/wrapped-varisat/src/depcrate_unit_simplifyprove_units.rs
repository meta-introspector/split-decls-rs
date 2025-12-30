// Generated macro for prove_units (function)
macro_rules! Depcrate_unit_simplifyprove_units {
() => {
// Module: crate::unit_simplify
// Provides: {"prove_units"}
// Dependencies: {}
# [doc = " Remove satisfied clauses and false literals."] pub fn prove_units < 'a > (mut ctx : partial ! (Context <'a >, mut ImplGraphP , mut ProofP <'a >, mut SolverStateP , mut TrailP , AssignmentP , ClauseAllocP , VariablesP ,) ,) -> bool { let mut new_unit = false ; if ctx . part (TrailP) . current_level () == 0 { let (impl_graph , mut ctx) = ctx . split_part_mut (ImplGraphP) ; let mut unit_proofs = vec ! [] ; let (trail , mut ctx) = ctx . split_part_mut (TrailP) ; for & lit in trail . trail () { new_unit = true ; let ctx_lits = ctx . borrow () ; let reason = impl_graph . reason (lit . var ()) ; if ! reason . is_unit () { let lits = impl_graph . reason (lit . var ()) . lits (& ctx_lits) ; let hash = clause_hash (lits) ^ lit_hash (lit) ; unit_proofs . push ((lit , hash)) ; } impl_graph . update_removed_unit (lit . var ()) ; } trail . clear () ; if ! unit_proofs . is_empty () { proof :: add_step (ctx . borrow () , true , & ProofStep :: UnitClauses { units : & unit_proofs , } ,) ; } } new_unit }
};
}
