// Generated macro for Key (struct)
macro_rules! Depcrate_keyKey {
() => {
// Module: crate::key
// Provides: {"Key"}
// Dependencies: {}
# [doc = " For Key/[`Value`][crate::Value] pairs under a [`Table`][crate::Table] header or inside an"] # [doc = " [`InlineTable`][crate::InlineTable]"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```notrust"] # [doc = " [dependencies.\"nom\"]"] # [doc = " version = \"5.0\""] # [doc = " 'literal key' = \"nonsense\""] # [doc = " \"basic string key\" = 42"] # [doc = " ```"] # [doc = ""] # [doc = " There are 3 types of keys:"] # [doc = ""] # [doc = " 1. Bare keys (`version` and `dependencies`)"] # [doc = ""] # [doc = " 2. Basic quoted keys (`\"basic string key\"` and `\"nom\"`)"] # [doc = ""] # [doc = " 3. Literal quoted keys (`'literal key'`)"] # [doc = ""] # [doc = " For details see [toml spec](https://github.com/toml-lang/toml/#keyvalue-pair)."] # [doc = ""] # [doc = " To parse a key use `FromStr` trait implementation: `\"string\".parse::<Key>()`."] # [derive (Debug)] pub struct Key { key : String , pub (crate) repr : Option < Repr > , pub (crate) leaf_decor : Decor , pub (crate) dotted_decor : Decor , }
};
}
