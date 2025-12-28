macro_rules! deps {
    () => {
        AstIdNode!();
    };
}

macro_rules! macro_40 {
    () => {
        deps!();
        register_has_name_ast_id ! { impl AstIdNode for Enum = name , Struct = name , Union = name , ExternCrate = name_ref , MacroDef = name , MacroRules = name , Module = name , Static = name , Trait = name }
    };
}

macro_40!();