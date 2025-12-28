macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_563 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A struct or struct variant pattern: `Variant { x, y, .. }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatStruct { pub attrs : Vec < Attribute >, pub qself : Option < QSelf >, pub path : Path , pub brace_token : token :: Brace , pub fields : Punctuated < FieldPat , Token ! [,] >, pub rest : Option < PatRest >, } }
    };
}

macro_563!();