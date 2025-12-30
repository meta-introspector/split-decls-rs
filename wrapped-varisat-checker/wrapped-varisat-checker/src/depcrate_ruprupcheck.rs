// Generated macro for RupCheck (struct)
macro_rules! Depcrate_rupRupCheck {
() => {
// Module: crate::rup
// Provides: {"RupCheck"}
// Dependencies: {}
# [derive (Default)] pub struct RupCheck { # [doc = " Stores overwritten values in `unit_clauses` to undo assignments."] trail : Vec < (Lit , Option < UnitClause >) > , # [doc = " Involved clauses during the last check."] trace : Vec < TraceItem > , # [doc = " Edges of the trace implication graph."] trace_edges : Vec < LitIdx > , # [doc = " Just the ids of `trace`."] pub trace_ids : Vec < u64 > , }
};
}
