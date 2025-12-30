// Generated macro for WitConfig (struct)
macro_rules! Depcrate_configWitConfig {
() => {
// Module: crate::config
// Provides: {"WitConfig"}
// Dependencies: {}
# [doc = " Configuration found in `*.wit` file either in codegen tests or in `test.wit`"] # [doc = " files for runtime tests."] # [derive (Clone , Default , Deserialize)] # [serde (deny_unknown_fields , rename_all = "kebab-case")] pub struct WitConfig { # [doc = " Indicates that this WIT test uses the component model async features"] # [doc = " and/or proposal."] # [doc = ""] # [doc = " This can be used to help expect failure in languages that do not yet"] # [doc = " support this proposal."] # [serde (default , rename = "async")] pub async_ : bool , # [doc = " Whether or not this test uses `error-context`"] # [serde (default)] pub error_context : bool , # [doc = " When set to `true` disables the passing of per-language default bindgen"] # [doc = " arguments. For example with Rust it avoids passing `--generate-all` by"] # [doc = " default to bindings generation."] pub default_bindgen_args : Option < bool > , # [doc = " Name of the world for the \"runner\" component, and note that this affects"] # [doc = " filenames as well."] pub runner : Option < String > , # [doc = " List of worlds for \"test\" components. This affects filenames and these"] # [doc = " are all available to import to the \"runner\"."] pub dependencies : Option < StringList > , # [doc = " Path to a `*.wac` file to specify how composition is done."] pub wac : Option < String > , }
};
}
