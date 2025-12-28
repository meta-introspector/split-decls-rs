macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! Fragment {
    () => {
        deps!();
        pub enum Fragment { # [doc = " Tokens that can be used as an expression."] Expr (TokenStream) , # [doc = " Tokens that can be used inside a block. The surrounding curly braces are"] # [doc = " not part of these tokens."] Block (TokenStream) , }
    };
}

Fragment!();