macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_747 {
    () => {
        deps!();
        ast_struct ! { # [doc = " An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or"] # [doc = " a lifetime."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeImplTrait { pub impl_token : Token ! [impl] , pub bounds : Punctuated < TypeParamBound , Token ! [+] >, } }
    };
}

macro_747!()