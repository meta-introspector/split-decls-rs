macro_rules! macro_723 {
    () => {
        ast_struct ! { # [doc = " The expression assigned in a local `let` binding, including optional"] # [doc = " diverging `else` block."] # [doc = ""] # [doc = " `LocalInit` represents `= s.parse()?` in `let x: u64 = s.parse()?` and"] # [doc = " `= r else { return }` in `let Ok(x) = r else { return }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct LocalInit { pub eq_token : Token ! [=] , pub expr : Box < Expr >, pub diverge : Option < (Token ! [else] , Box < Expr >) >, } }
    };
}

macro_723!();