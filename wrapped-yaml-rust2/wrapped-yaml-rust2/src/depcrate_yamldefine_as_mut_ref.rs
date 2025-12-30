// Generated macro for define_as_mut_ref (macro)
macro_rules! Depcrate_yamldefine_as_mut_ref {
() => {
// Module: crate::yaml
// Provides: {"define_as_mut_ref"}
// Dependencies: {}
macro_rules ! define_as_mut_ref (($ name : ident , $ t : ty , $ yt : ident) => (# [doc = " Get a mutable reference to the inner object in the YAML enum if it is a `$t`."] # [doc = ""] # [doc = " # Return"] # [doc = " If the variant of `self` is `Yaml::$yt`, return `Some(&mut $t)` with the `$t` contained."] # [doc = " Otherwise, return `None`."] # [must_use] pub fn $ name (& mut self) -> Option <$ t > { match * self { Yaml ::$ yt (ref mut v) => Some (v) , _ => None } }) ;) ;
};
}
