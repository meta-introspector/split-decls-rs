// Generated macro for macro_817 (macro)
macro_rules! Depcrate_pathmacro_817 {
() => {
// Module: crate::path
// Provides: {"macro_817"}
// Dependencies: {}
ast_enum ! { # [doc = " An individual generic argument, like `'a`, `T`, or `Item = T`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] # [non_exhaustive] pub enum GenericArgument { # [doc = " A lifetime argument."] Lifetime (Lifetime) , # [doc = " A type argument."] Type (Type) , # [doc = " A const expression. Must be inside of a block."] # [doc = ""] # [doc = " NOTE: Identity expressions are represented as Type arguments, as"] # [doc = " they are indistinguishable syntactically."] Const (Expr) , # [doc = " A binding (equality constraint) on an associated type: the `Item ="] # [doc = " u8` in `Iterator<Item = u8>`."] AssocType (AssocType) , # [doc = " An equality constraint on an associated constant: the `PANIC ="] # [doc = " false` in `Trait<PANIC = false>`."] AssocConst (AssocConst) , # [doc = " An associated type bound: `Iterator<Item: Display>`."] Constraint (Constraint) , } }
};
}
