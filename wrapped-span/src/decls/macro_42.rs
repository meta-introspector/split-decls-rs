macro_rules! deps {
    () => {
        AstIdNode!();
    };
}

macro_rules! macro_42 {
    () => {
        deps!();
        register_assoc_item_ast_id ! { impl AstIdNode for Variant = | it : ast :: Variant | it . name () , Const = | it : ast :: Const | it . name () , Fn = | it : ast :: Fn | it . name () , MacroCall = | it : ast :: MacroCall | it . path () . and_then (| path | path . segment () ?. name_ref ()) , TypeAlias = | it : ast :: TypeAlias | it . name () }
    };
}

macro_42!()