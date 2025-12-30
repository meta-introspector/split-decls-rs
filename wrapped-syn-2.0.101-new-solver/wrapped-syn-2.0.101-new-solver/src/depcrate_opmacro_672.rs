// Generated macro for macro_672 (macro)
macro_rules! Depcrate_opmacro_672 {
() => {
// Module: crate::op
// Provides: {"macro_672"}
// Dependencies: {}
ast_enum ! { # [doc = " A unary operator: `*`, `!`, `-`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] # [non_exhaustive] pub enum UnOp { # [doc = " The `*` operator for dereferencing"] Deref (Token ! [*]) , # [doc = " The `!` operator for logical inversion"] Not (Token ! [!]) , # [doc = " The `-` operator for negation"] Neg (Token ! [-]) , } }
};
}
