macro_rules! macro_351 {
    () => {
        ast_struct ! { # [doc = " A macro invocation, which includes `macro_rules!` definitions."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemMacro { pub attrs : Vec < Attribute >, # [doc = " The `example` in `macro_rules! example { ... }`."] pub ident : Option < Ident >, pub mac : Macro , pub semi_token : Option < Token ! [;] >, } }
    };
}

macro_351!()