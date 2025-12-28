macro_rules! macro_255 {
    () => {
        # [cfg (feature = "full")] ast_struct ! { # [doc = " One arm of a `match` expression: `0..=10 => { return true; }`."] # [doc = ""] # [doc = " As in:"] # [doc = ""] # [doc = " ```"] # [doc = " # fn f() -> bool {"] # [doc = " #     let n = 0;"] # [doc = " match n {"] # [doc = "     0..=10 => {"] # [doc = "         return true;"] # [doc = "     }"] # [doc = "     // ..."] # [doc = "     # _ => {}"] # [doc = " }"] # [doc = " #   false"] # [doc = " # }"] # [doc = " ```"] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct Arm { pub attrs : Vec < Attribute >, pub pat : Pat , pub guard : Option < (Token ! [if] , Box < Expr >) >, pub fat_arrow_token : Token ! [=>] , pub body : Box < Expr >, pub comma : Option < Token ! [,] >, } }
    };
}

macro_255!()