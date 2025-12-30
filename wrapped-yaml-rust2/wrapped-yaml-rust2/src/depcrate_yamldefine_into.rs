// Generated macro for define_into (macro)
macro_rules! Depcrate_yamldefine_into {
() => {
// Module: crate::yaml
// Provides: {"define_into"}
// Dependencies: {}
macro_rules ! define_into (($ name : ident , $ t : ty , $ yt : ident) => (# [doc = " Get the inner object in the YAML enum if it is a `$t`."] # [doc = ""] # [doc = " # Return"] # [doc = " If the variant of `self` is `Yaml::$yt`, return `Some($t)` with the `$t` contained. Otherwise,"] # [doc = " return `None`."] # [must_use] pub fn $ name (self) -> Option <$ t > { match self { Yaml ::$ yt (v) => Some (v) , _ => None } }) ;) ;
};
}
