macro_rules! macro_373 {
    () => {
        ast_struct ! { # [doc = " A foreign static item in an `extern` block: `static ext: u8`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ForeignItemStatic { pub attrs : Vec < Attribute >, pub vis : Visibility , pub static_token : Token ! [static] , pub mutability : StaticMutability , pub ident : Ident , pub colon_token : Token ! [:] , pub ty : Box < Type >, pub semi_token : Token ! [;] , } }
    };
}

macro_373!();