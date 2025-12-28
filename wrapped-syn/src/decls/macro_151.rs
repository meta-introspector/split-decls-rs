macro_rules! macro_151 {
    () => {
        ast_struct ! { # [doc = " Data structure sent to a `proc_macro_derive` macro."] # [cfg_attr (docsrs , doc (cfg (feature = "derive")))] pub struct DeriveInput { pub attrs : Vec < Attribute >, pub vis : Visibility , pub ident : Ident , pub generics : Generics , pub data : Data , } }
    };
}

macro_151!()