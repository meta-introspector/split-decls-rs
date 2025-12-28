macro_rules! macro_380 {
    () => {
        ast_struct ! { # [doc = " A macro invocation within the definition of a trait."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct TraitItemMacro { pub attrs : Vec < Attribute >, pub mac : Macro , pub semi_token : Option < Token ! [;] >, } }
    };
}

macro_380!()