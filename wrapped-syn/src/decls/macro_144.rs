macro_rules! macro_144 {
    () => {
        ast_struct ! { # [doc = " A field of a struct or enum variant."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct Field { pub attrs : Vec < Attribute >, pub vis : Visibility , pub mutability : FieldMutability , # [doc = " Name of the field, if any."] # [doc = ""] # [doc = " Fields of tuple structs have no names."] pub ident : Option < Ident >, pub colon_token : Option < Token ! [:] >, pub ty : Type , } }
    };
}

macro_144!()