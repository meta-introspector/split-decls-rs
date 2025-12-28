macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! macro_325 {
    () => {
        deps!();
        # [cfg (feature = "full")] ast_enum ! { # [doc = " Single parameter in a precise capturing bound."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] # [non_exhaustive] pub enum CapturedParam { # [doc = " A lifetime parameter in precise capturing bound: `fn f<'a>() -> impl"] # [doc = " Trait + use<'a>`."] Lifetime (Lifetime) , # [doc = " A type parameter or const generic parameter in precise capturing"] # [doc = " bound: `fn f<T>() -> impl Trait + use<T>` or `fn f<const K: T>() ->"] # [doc = " impl Trait + use<K>`."] Ident (Ident) , } }
    };
}

macro_325!()