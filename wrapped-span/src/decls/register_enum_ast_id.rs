macro_rules! deps {
    () => {
        AstIdNode!();
    };
}

macro_rules! register_enum_ast_id {
    () => {
        deps!();
        macro_rules ! register_enum_ast_id { (impl $ AstIdNode : ident for $ ($ ident : ident) ,+) => { $ (impl $ AstIdNode for ast ::$ ident { }) + } ; }
    };
}

register_enum_ast_id!();