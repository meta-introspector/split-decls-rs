// Generated macro for tests (module)
macro_rules! Depcrate_ast_reflectortests {
() => {
// Module: crate::ast_reflector
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_probe_creation () { let probes = create_example_probes () ; assert_eq ! (probes . len () , 4) ; assert ! (probes [0] . enabled) ; } # [test] fn test_filter_matching () { let reflector = AstReflector :: new () ; let context = ProbeContext { file_path : PathBuf :: from ("test.rs") , node_name : "debug_function" . to_string () , node_type : AstNodeType :: Function , attributes : vec ! [] , visibility : "pub" . to_string () , complexity : 3.0 , } ; let filter = ProbeFilter { name_pattern : Some ("debug" . to_string ()) , visibility : None , attributes : vec ! [] , contains_text : None , complexity_threshold : None , layer : None , } ; assert ! (reflector . matches_filter (& filter , & context)) ; } }
};
}
