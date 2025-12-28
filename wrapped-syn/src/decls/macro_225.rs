macro_rules! macro_225 {
    () => {
        ast_struct ! { # [doc = " Address-of operation: `&raw const place` or `&raw mut place`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprRawAddr # full { pub attrs : Vec < Attribute >, pub and_token : Token ! [&] , pub raw : Token ! [raw] , pub mutability : PointerMutability , pub expr : Box < Expr >, } }
    };
}

macro_225!();