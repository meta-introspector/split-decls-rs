macro_rules! macro_486 {
    () => {
        ast_enum ! { # [doc = " A unary operator: `*`, `!`, `-`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] # [non_exhaustive] pub enum UnOp { # [doc = " The `*` operator for dereferencing"] Deref (Token ! [*]) , # [doc = " The `!` operator for logical inversion"] Not (Token ! [!]) , # [doc = " The `-` operator for negation"] Neg (Token ! [-]) , } }
    };
}

macro_486!();