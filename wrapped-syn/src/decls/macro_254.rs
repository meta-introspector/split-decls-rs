macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! macro_254 {
    () => {
        deps!();
        # [cfg (feature = "full")] ast_struct ! { # [doc = " A lifetime labeling a `for`, `while`, or `loop`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct Label { pub name : Lifetime , pub colon_token : Token ! [:] , } }
    };
}

macro_254!();