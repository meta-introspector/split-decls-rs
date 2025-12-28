macro_rules! deps {
    () => {
        Fragment!();
    };
}

macro_rules! Expr {
    () => {
        deps!();
        # [doc = " Interpolate a fragment in place of an expression. This involves surrounding"] # [doc = " Block fragments in curly braces."] pub struct Expr (pub Fragment) ;
    };
}

Expr!();