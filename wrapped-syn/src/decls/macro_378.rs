macro_rules! macro_378 {
    () => {
        ast_struct ! { # [doc = " An associated function within the definition of a trait."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct TraitItemFn { pub attrs : Vec < Attribute >, pub sig : Signature , pub default : Option < Block >, pub semi_token : Option < Token ! [;] >, } }
    };
}

macro_378!();