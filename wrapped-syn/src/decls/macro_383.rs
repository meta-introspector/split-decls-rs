macro_rules! macro_383 {
    () => {
        ast_struct ! { # [doc = " An associated function within an impl block."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ImplItemFn { pub attrs : Vec < Attribute >, pub vis : Visibility , pub defaultness : Option < Token ! [default] >, pub sig : Signature , pub block : Block , } }
    };
}

macro_383!()