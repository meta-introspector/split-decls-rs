macro_rules! Body {
    () => {
        struct Body < 'a > { brace_token : syn :: token :: Brace , stmts : & 'a [TokenStream] , }
    };
}

Body!()