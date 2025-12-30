// Generated macro for response_no_constraints (function)
macro_rules! Depcrate_solve_search_graphresponse_no_constraints {
() => {
// Module: crate::solve::search_graph
// Provides: {"response_no_constraints"}
// Dependencies: {}
fn response_no_constraints < I : Interner > (cx : I , input : CanonicalInput < I > , certainty : Certainty ,) -> QueryResult < I > { Ok (super :: response_no_constraints_raw (cx , input . canonical . max_universe , input . canonical . variables , certainty ,)) }
};
}
