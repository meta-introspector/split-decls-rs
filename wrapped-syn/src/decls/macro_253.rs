macro_rules! macro_253 {
    () => {
        ast_struct ! { # [doc = " A field-value pair in a struct literal."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct FieldValue { pub attrs : Vec < Attribute >, pub member : Member , # [doc = " The colon in `Struct { x: x }`. If written in shorthand like"] # [doc = " `Struct { x }`, there is no colon."] pub colon_token : Option < Token ! [:] >, pub expr : Expr , } }
    };
}

macro_253!()