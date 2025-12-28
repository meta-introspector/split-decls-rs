macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_370 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A braced group of imports in a `use` item: `{A, B, C}`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct UseGroup { pub brace_token : token :: Brace , pub items : Punctuated < UseTree , Token ! [,] >, } }
    };
}

macro_370!();