// Generated macro for LangConfig (struct)
macro_rules! Depcrate_cppLangConfig {
() => {
// Module: crate::cpp
// Provides: {"LangConfig"}
// Dependencies: {}
# [doc = " C/C++-specific configuration of component files"] # [derive (Default , Deserialize)] # [serde (deny_unknown_fields)] struct LangConfig { # [doc = " Space-separated list or array of compiler flags to pass."] # [serde (default)] cflags : StringList , }
};
}
