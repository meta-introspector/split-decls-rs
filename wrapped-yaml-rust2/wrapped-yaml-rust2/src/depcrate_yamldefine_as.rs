// Generated macro for define_as (macro)
macro_rules! Depcrate_yamldefine_as {
() => {
// Module: crate::yaml
// Provides: {"define_as"}
// Dependencies: {}
macro_rules ! define_as (($ name : ident , $ t : ident , $ yt : ident) => (# [doc = " Get a copy of the inner object in the YAML enum if it is a `$t`."] # [doc = ""] # [doc = " # Return"] # [doc = " If the variant of `self` is `Yaml::$yt`, return `Some($t)` with a copy of the `$t` contained."] # [doc = " Otherwise, return `None`."] # [must_use] pub fn $ name (& self) -> Option <$ t > { match * self { Yaml ::$ yt (v) => Some (v) , _ => None } }) ;) ;
};
}
