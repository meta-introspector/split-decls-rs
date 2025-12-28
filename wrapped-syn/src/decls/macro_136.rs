macro_rules! macro_136 {
    () => {
        ast_struct ! { # [doc = " An enum variant."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct Variant { pub attrs : Vec < Attribute >, # [doc = " Name of the variant."] pub ident : Ident , # [doc = " Content stored in the variant."] pub fields : Fields , # [doc = " Explicit discriminant: `Variant = 1`"] pub discriminant : Option < (Token ! [=] , Expr) >, } }
    };
}

macro_136!();