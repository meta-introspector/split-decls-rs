// Generated macro for RuntimeTestConfig (struct)
macro_rules! Depcrate_configRuntimeTestConfig {
() => {
// Module: crate::config
// Provides: {"RuntimeTestConfig"}
// Dependencies: {}
# [doc = " Configuration that can be placed at the top of runtime tests in source"] # [doc = " language files."] # [doc = ""] # [doc = " This is a union of language-agnostic and language-specific configuration."] # [doc = " Language-agnostic configuration can be bindings generator arguments:"] # [doc = ""] # [doc = " ```toml"] # [doc = " args = '--foo --bar'"] # [doc = " #  or ..."] # [doc = " args = ['--foo', '--bar']"] # [doc = " ```"] # [doc = ""] # [doc = " but languages may each have their own configuration:"] # [doc = ""] # [doc = " ```toml"] # [doc = " [lang]"] # [doc = " rustflags = '-O'"] # [doc = " ```"] # [doc = ""] # [doc = " The `Component::deserialize_lang_config` helper is used to deserialize the"] # [doc = " `lang` field here."] # [derive (Default , Deserialize)] # [serde (deny_unknown_fields , rename_all = "kebab-case")] pub struct RuntimeTestConfig < T = HashMap < String , toml :: Value > > { # [doc = " Extra command line arguments to pass to the language-specific bindings"] # [doc = " generator."] # [doc = ""] # [doc = " This is either a string which is whitespace delimited or it's an array"] # [doc = " of strings. By default no extra arguments are passed."] # [serde (default)] pub args : StringList , # [doc = " Language-specific configuration"] pub lang : Option < T > , }
};
}
