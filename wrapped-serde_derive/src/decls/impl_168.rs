macro_rules! deps {
    () => {
        Expr!();
        Fragment!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl ToTokens for Expr { fn to_tokens (& self , out : & mut TokenStream) { match & self . 0 { Fragment :: Expr (expr) => expr . to_tokens (out) , Fragment :: Block (block) => { token :: Brace :: default () . surround (out , | out | block . to_tokens (out)) ; } } } }
    };
}

impl_168!()