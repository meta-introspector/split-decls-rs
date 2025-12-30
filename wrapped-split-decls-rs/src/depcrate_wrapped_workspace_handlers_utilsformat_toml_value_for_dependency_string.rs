// Generated macro for format_toml_value_for_dependency_string (function)
macro_rules! Depcrate_wrapped_workspace_handlers_utilsformat_toml_value_for_dependency_string {
() => {
// Module: crate::wrapped_workspace_handlers::utils
// Provides: {"format_toml_value_for_dependency_string"}
// Dependencies: {}
pub fn format_toml_value_for_dependency_string (value : & Value) -> String { match value { Value :: String (s) => format ! ("\"{}\"" , s) , Value :: Integer (i) => i . to_string () , Value :: Float (f) => f . to_string () , Value :: Boolean (b) => b . to_string () , Value :: Datetime (d) => format ! ("\"{}\"" , d) , Value :: Array (arr) => { let elements : Vec < String > = arr . iter () . map (format_toml_value_for_dependency_string) . collect () ; format ! ("[{}]" , elements . join (", ")) } , Value :: Table (table) => { let mut parts = Vec :: new () ; for (key , val) in table . iter () { parts . push (format ! ("{} = {}" , key , format_toml_value_for_dependency_string (val))) ; } format ! ("{{ {} }}" , parts . join (", ")) } , _ => value . to_string () , } }
};
}
