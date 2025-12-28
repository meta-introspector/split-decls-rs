macro_rules! deps {
    () => {
        Fragment!();
        Expr!();
    };
}

macro_rules! quote_expr {
    () => {
        deps!();
        macro_rules ! quote_expr { ($ ($ tt : tt) *) => { $ crate :: fragment :: Fragment :: Expr (quote ! ($ ($ tt) *)) } }
    };
}

quote_expr!()