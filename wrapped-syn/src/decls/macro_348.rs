macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! macro_348 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A free-standing function: `fn process(n: usize) -> Result<()> { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemFn { pub attrs : Vec < Attribute >, pub vis : Visibility , pub sig : Signature , pub block : Box < Block >, } }
    };
}

macro_348!();