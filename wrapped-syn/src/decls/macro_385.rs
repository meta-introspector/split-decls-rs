macro_rules! macro_385 {
    () => {
        ast_struct ! { # [doc = " A macro invocation within an impl block."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ImplItemMacro { pub attrs : Vec < Attribute >, pub mac : Macro , pub semi_token : Option < Token ! [;] >, } }
    };
}

macro_385!()