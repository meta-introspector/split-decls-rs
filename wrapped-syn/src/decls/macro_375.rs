macro_rules! macro_375 {
    () => {
        ast_struct ! { # [doc = " A macro invocation within an extern block."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ForeignItemMacro { pub attrs : Vec < Attribute >, pub mac : Macro , pub semi_token : Option < Token ! [;] >, } }
    };
}

macro_375!()