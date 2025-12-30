// Generated macro for response_no_constraints_raw (function)
macro_rules! Depcrate_solveresponse_no_constraints_raw {
() => {
// Module: crate::solve
// Provides: {"response_no_constraints_raw"}
// Dependencies: {}
fn response_no_constraints_raw < I : Interner > (cx : I , max_universe : ty :: UniverseIndex , variables : I :: CanonicalVarKinds , certainty : Certainty ,) -> CanonicalResponse < I > { ty :: Canonical { max_universe , variables , value : Response { var_values : ty :: CanonicalVarValues :: make_identity (cx , variables) , external_constraints : cx . mk_external_constraints (ExternalConstraintsData :: default ()) , certainty , } , } }
};
}
