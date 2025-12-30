// Generated macro for reconstruct_global_model (function)
macro_rules! Depcrate_modelreconstruct_global_model {
() => {
// Module: crate::model
// Provides: {"reconstruct_global_model"}
// Dependencies: {}
pub fn reconstruct_global_model < 'a > (mut ctx : partial ! (Context <'a >, mut ModelP , mut ProofP <'a >, mut SolverStateP , mut TmpDataP , AssignmentP , VariablesP) ,) { let (variables , mut ctx) = ctx . split_part (VariablesP) ; let (model , mut ctx) = ctx . split_part_mut (ModelP) ; let (tmp , mut ctx) = ctx . split_part_mut (TmpDataP) ; let models_in_proof = ctx . part (ProofP) . models_in_proof () ; tmp . lits . clear () ; model . assignment . clear () ; model . assignment . resize (variables . global_watermark () , None) ; for global_var in variables . global_var_iter () { let value = if let Some (solver_var) = variables . solver_from_global () . get (global_var) { ctx . part (AssignmentP) . var_value (solver_var) } else { Some (variables . var_data_global (global_var) . unit . unwrap_or (false)) } ; model . assignment [global_var . index ()] = value ; if models_in_proof { if let Some (value) = value { tmp . lits . push (global_var . lit (value)) } } } if models_in_proof { proof :: add_step (ctx . borrow () , false , & ProofStep :: Model { assignment : & tmp . lits , } ,) ; } ctx . part_mut (SolverStateP) . sat_state = SatState :: Sat ; }
};
}
