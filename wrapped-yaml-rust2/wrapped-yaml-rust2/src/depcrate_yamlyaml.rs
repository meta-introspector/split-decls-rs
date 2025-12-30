// Generated macro for Yaml (enum)
macro_rules! Depcrate_yamlYaml {
() => {
// Module: crate::yaml
// Provides: {"Yaml"}
// Dependencies: {}
# [doc = " A YAML node is stored as this `Yaml` enumeration, which provides an easy way to"] # [doc = " access your YAML document."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use yaml_rust2::Yaml;"] # [doc = " let foo = Yaml::from_str(\"-123\"); // convert the string to the appropriate YAML type"] # [doc = " assert_eq!(foo.as_i64().unwrap(), -123);"] # [doc = ""] # [doc = " // iterate over an Array"] # [doc = " let vec = Yaml::Array(vec![Yaml::Integer(1), Yaml::Integer(2)]);"] # [doc = " for v in vec.as_vec().unwrap() {"] # [doc = "     assert!(v.as_i64().is_some());"] # [doc = " }"] # [doc = " ```"] # [derive (Clone , PartialEq , PartialOrd , Debug , Eq , Ord , Hash)] pub enum Yaml { # [doc = " Float types are stored as String and parsed on demand."] # [doc = " Note that `f64` does NOT implement Eq trait and can NOT be stored in `BTreeMap`."] Real (String) , # [doc = " YAML int is stored as i64."] Integer (i64) , # [doc = " YAML scalar."] String (String) , # [doc = " YAML bool, e.g. `true` or `false`."] Boolean (bool) , # [doc = " YAML array, can be accessed as a [`Vec`]."] Array (Array) , # [doc = " YAML hash, can be accessed as a [`LinkedHashMap`]."] # [doc = ""] # [doc = " Insertion order will match the order of insertion into the map."] Hash (Hash) , # [doc = " Alias, not fully supported yet."] Alias (usize) , # [doc = " YAML null, e.g. `null` or `~`."] Null , # [doc = " Accessing a nonexistent node via the Index trait returns `BadValue`. This"] # [doc = " simplifies error handling in the calling code. Invalid type conversion also"] # [doc = " returns `BadValue`."] BadValue , }
};
}
