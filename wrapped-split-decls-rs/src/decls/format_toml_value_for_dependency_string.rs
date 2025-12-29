// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "format_toml_value_for_dependency_string",
decl_type: "function",
source_file: "./src/wrapped_workspace_handlers/utils.rs",
source_crate: ".",
deps: [],
uses: ["Datetime", "Boolean", "Array", "Vec", "Value", "String", "Table", "Float", "Integer"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! format_toml_value_for_dependency_string {
    () => {
        pub fn format_toml_value_for_dependency_string (value : & Value) -> String { match value { Value :: String (s) => format ! ("\"{}\"" , s) , Value :: Integer (i) => i . to_string () , Value :: Float (f) => f . to_string () , Value :: Boolean (b) => b . to_string () , Value :: Datetime (d) => format ! ("\"{}\"" , d) , Value :: Array (arr) => { let elements : Vec < String > = arr . iter () . map (format_toml_value_for_dependency_string) . collect () ; format ! ("[{}]" , elements . join (", ")) } , Value :: Table (table) => { let mut parts = Vec :: new () ; for (key , val) in table . iter () { parts . push (format ! ("{} = {}" , key , format_toml_value_for_dependency_string (val))) ; } format ! ("{{ {} }}" , parts . join (", ")) } , _ => value . to_string () , } }
    };
}

format_toml_value_for_dependency_string!();