// Generated macro for get_features_doc (function)
macro_rules! Depcrate_generatorget_features_doc {
() => {
// Module: crate::generator
// Provides: {"get_features_doc"}
// Dependencies: {}
fn get_features_doc (options : & Options , name : String) -> Option < String > { let mut features = BTreeSet :: new () ; features . insert (name) ; required_doc_string (options , & features) }
};
}
