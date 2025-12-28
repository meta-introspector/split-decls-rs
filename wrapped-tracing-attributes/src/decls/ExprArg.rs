macro_rules! ExprArg {
    () => {
        struct ExprArg < T > { value : Expr , _p : std :: marker :: PhantomData < T > , }
    };
}

ExprArg!();