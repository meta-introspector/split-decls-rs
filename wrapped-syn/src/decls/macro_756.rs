macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_756 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a"] # [doc = " trait or a lifetime."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeTraitObject { pub dyn_token : Option < Token ! [dyn] >, pub bounds : Punctuated < TypeParamBound , Token ! [+] >, } }
    };
}

macro_756!();