macro_rules! macro_557 {
    () => {
        ast_struct ! { # [doc = " A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`."] # [doc = ""] # [doc = " It may also be a unit struct or struct variant (e.g. `None`), or a"] # [doc = " constant; these cannot be distinguished syntactically."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatIdent { pub attrs : Vec < Attribute >, pub by_ref : Option < Token ! [ref] >, pub mutability : Option < Token ! [mut] >, pub ident : Ident , pub subpat : Option < (Token ! [@] , Box < Pat >) >, } }
    };
}

macro_557!();