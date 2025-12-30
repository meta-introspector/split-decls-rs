// Generated macro for YamlEmitter (struct)
macro_rules! Depcrate_emitterYamlEmitter {
() => {
// Module: crate::emitter
// Provides: {"YamlEmitter"}
// Dependencies: {}
# [doc = " The YAML serializer."] # [doc = ""] # [doc = " ```"] # [doc = " # use yaml_rust2::{YamlLoader, YamlEmitter};"] # [doc = " let input_string = \"a: b\\nc: d\";"] # [doc = " let yaml = YamlLoader::load_from_str(input_string).unwrap();"] # [doc = ""] # [doc = " let mut output = String::new();"] # [doc = " YamlEmitter::new(&mut output).dump(&yaml[0]).unwrap();"] # [doc = ""] # [doc = " assert_eq!(output, r#\"---"] # [doc = " a: b"] # [doc = " c: d\"#);"] # [doc = " ```"] # [allow (clippy :: module_name_repetitions)] pub struct YamlEmitter < 'a > { writer : & 'a mut dyn fmt :: Write , best_indent : usize , compact : bool , level : isize , multiline_strings : bool , }
};
}
