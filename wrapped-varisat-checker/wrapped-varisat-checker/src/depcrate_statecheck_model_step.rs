// Generated macro for check_model_step (function)
macro_rules! Depcrate_statecheck_model_step {
() => {
// Module: crate::state
// Provides: {"check_model_step"}
// Dependencies: {}
# [doc = " Check a Model step"] fn check_model_step < 'a > (mut ctx : partial ! (Context <'a >, mut ProcessingP <'a >, CheckerStateP , ClausesP , VariablesP) , model : & [Lit] ,) -> Result < () , CheckerError > { let mut assignments = HashSet :: default () ; for & lit in model . iter () { if let Some ((false , _)) = ctx . part (ClausesP) . lit_value (lit) { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , format ! ("model assignment conflicts with unit clause {:?}" , ! lit) ,)) ; } if assignments . contains (& ! lit) { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , format ! ("model contains conflicting assignment {:?}" , ! lit) ,)) ; } assignments . insert (lit) ; } for & lit in ctx . part (CheckerStateP) . assumptions . iter () { if ! assignments . contains (& lit) { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , format ! ("model does not contain assumption {:?}" , lit) ,)) ; } } for (_ , candidates) in ctx . part (ClausesP) . clauses . iter () { for clause in candidates . iter () { let lits = clause . lits . slice (& ctx . part (ClausesP) . literal_buffer) ; if ! lits . iter () . any (| lit | assignments . contains (& lit)) { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , format ! ("model does not satisfy clause {:?}" , lits) ,)) ; } } } process_step (ctx . borrow () , & CheckedProofStep :: Model { assignment : model }) ? ; Ok (()) }
};
}
