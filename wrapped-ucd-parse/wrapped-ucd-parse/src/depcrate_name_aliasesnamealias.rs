// Generated macro for NameAlias (struct)
macro_rules! Depcrate_name_aliasesNameAlias {
() => {
// Module: crate::name_aliases
// Provides: {"NameAlias"}
// Dependencies: {}
# [doc = " A single row in the `NameAliases.txt` file."] # [doc = ""] # [doc = " Note that there are multiple rows for some codepoint. Each row provides a"] # [doc = " new alias."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct NameAlias { # [doc = " The codepoint corresponding to this row."] pub codepoint : Codepoint , # [doc = " The alias."] pub alias : String , # [doc = " The label of this alias."] pub label : NameAliasLabel , }
};
}
