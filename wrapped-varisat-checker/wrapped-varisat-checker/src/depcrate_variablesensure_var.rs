// Generated macro for ensure_var (function)
macro_rules! Depcrate_variablesensure_var {
() => {
// Module: crate::variables
// Provides: {"ensure_var"}
// Dependencies: {}
# [doc = " Ensure that a variable is present."] pub fn ensure_var (mut ctx : partial ! (Context , mut ClausesP , mut VariablesP) , var : Var) { let (variables , mut ctx) = ctx . split_part_mut (VariablesP) ; if variables . var_data . len () <= var . index () { variables . var_data . resize (var . index () + 1 , VarData :: default ()) ; variables . lit_data . resize ((var . index () + 1) * 2 , LitData :: default ()) ; ctx . part_mut (ClausesP) . unit_clauses . resize (var . index () + 1 , None) ; } }
};
}
