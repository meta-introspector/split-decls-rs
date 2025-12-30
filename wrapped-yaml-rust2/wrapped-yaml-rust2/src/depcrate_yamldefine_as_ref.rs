// Generated macro for define_as_ref (macro)
macro_rules! Depcrate_yamldefine_as_ref {
() => {
// Module: crate::yaml
// Provides: {"define_as_ref"}
// Dependencies: {}
macro_rules ! define_as_ref (($ name : ident , $ t : ty , $ yt : ident) => (# [doc = " Get a reference to the inner object in the YAML enum if it is a `$t`."] # [doc = ""] # [doc = " # Return"] # [doc = " If the variant of `self` is `Yaml::$yt`, return `Some(&$t)` with the `$t` contained. Otherwise,"] # [doc = " return `None`."] # [must_use] pub fn $ name (& self) -> Option <$ t > { match * self { Yaml ::$ yt (ref v) => Some (v) , _ => None } }) ;) ;
};
}
