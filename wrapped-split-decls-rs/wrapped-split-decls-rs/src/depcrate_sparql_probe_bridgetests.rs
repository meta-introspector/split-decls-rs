// Generated macro for tests (module)
macro_rules! Depcrate_sparql_probe_bridgetests {
() => {
// Module: crate::sparql_probe_bridge
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_sparql_probe_generation () { let mut generator = SparqlProbeGenerator :: new () ; generator . rdf_data . insert ("test_function" . to_string () , 12.5) ; let probes = generator . generate_complexity_wrapper_probes (5) ; assert ! (! probes . is_empty ()) ; assert_eq ! (probes [0] . node_type , AstNodeType :: Function) ; } }
};
}
