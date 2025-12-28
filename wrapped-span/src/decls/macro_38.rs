macro_rules! deps {
    () => {
        AstIdNode!();
    };
}

macro_rules! macro_38 {
    () => {
        deps!();
        register_enum_ast_id ! { impl AstIdNode for Item , AnyHasGenericParams , Adt , Macro , AssocItem }
    };
}

macro_38!()