macro_rules! deps {
    () => {
        Match!();
        Expr!();
        Fragment!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl ToTokens for Match { fn to_tokens (& self , out : & mut TokenStream) { match & self . 0 { Fragment :: Expr (expr) => { expr . to_tokens (out) ; < Token ! [,] > :: default () . to_tokens (out) ; } Fragment :: Block (block) => { token :: Brace :: default () . surround (out , | out | block . to_tokens (out)) ; } } } }
    };
}

impl_172!();