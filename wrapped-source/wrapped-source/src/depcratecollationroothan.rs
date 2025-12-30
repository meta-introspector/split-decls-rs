// Generated macro for CollationRootHan (enum)
macro_rules! DepcrateCollationRootHan {
() => {
// Module: crate
// Provides: {"CollationRootHan"}
// Dependencies: {}
# [doc = " Specifies the collation Han database to use."] # [doc = ""] # [doc = " Unihan is more precise but significantly increases data size. See"] # [doc = " <https://github.com/unicode-org/icu/blob/main/docs/userguide/icu::data/buildtool.md#collation-ucadata>"] # [derive (Debug , Copy , Clone , PartialEq , Eq , Default , serde :: Serialize , serde :: Deserialize)] # [non_exhaustive] pub enum CollationRootHan { # [doc = " Implicit"] # [serde (rename = "implicit")] # [default] Implicit , # [doc = " Unihan"] # [serde (rename = "unihan")] Unihan , }
};
}
