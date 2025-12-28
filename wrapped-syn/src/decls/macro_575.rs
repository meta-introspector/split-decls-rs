macro_rules! macro_575 {
    () => {
        ast_struct ! { # [doc = " A segment of a path together with any path arguments on that segment."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct PathSegment { pub ident : Ident , pub arguments : PathArguments , } }
    };
}

macro_575!()