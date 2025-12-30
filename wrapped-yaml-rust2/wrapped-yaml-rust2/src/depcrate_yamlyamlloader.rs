// Generated macro for YamlLoader (struct)
macro_rules! Depcrate_yamlYamlLoader {
() => {
// Module: crate::yaml
// Provides: {"YamlLoader"}
// Dependencies: {}
# [doc = " Main structure for quickly parsing YAML."] # [doc = ""] # [doc = " See [`YamlLoader::load_from_str`]."] # [derive (Default)] pub struct YamlLoader { # [doc = " The different YAML documents that are loaded."] docs : Vec < Yaml > , doc_stack : Vec < (Yaml , usize) > , key_stack : Vec < Yaml > , anchor_map : BTreeMap < usize , Yaml > , # [doc = " An error, if one was encountered."] error : Option < ScanError > , }
};
}
