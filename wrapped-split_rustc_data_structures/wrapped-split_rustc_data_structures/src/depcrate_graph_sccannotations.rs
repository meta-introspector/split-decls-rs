// Generated macro for Annotations (trait)
macro_rules! Depcrate_graph_sccAnnotations {
() => {
// Module: crate::graph::scc
// Provides: {"Annotations"}
// Dependencies: {}
# [doc = " An accumulator for annotations."] pub trait Annotations < N : Idx > { type Ann : Annotation ; type SccIdx : Idx + Ord ; fn new (& self , element : N) -> Self :: Ann ; fn annotate_scc (& mut self , scc : Self :: SccIdx , annotation : Self :: Ann) ; }
};
}
