macro_rules! deps {
    () => {
        Stmts!();
        Expr!();
        Fragment!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl ToTokens for Stmts { fn to_tokens (& self , out : & mut TokenStream) { match & self . 0 { Fragment :: Expr (expr) => expr . to_tokens (out) , Fragment :: Block (block) => block . to_tokens (out) , } } }
    };
}

impl_170!();