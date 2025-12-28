macro_rules! macro_352 {
    () => {
        ast_struct ! { # [doc = " A module or module declaration: `mod m` or `mod m { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemMod { pub attrs : Vec < Attribute >, pub vis : Visibility , pub unsafety : Option < Token ! [unsafe] >, pub mod_token : Token ! [mod] , pub ident : Ident , pub content : Option < (token :: Brace , Vec < Item >) >, pub semi : Option < Token ! [;] >, } }
    };
}

macro_352!()