macro_rules! Value {
    () => {
        enum Value { SameAsName , String (LitStr) , Env (LitStr , Macro) , Unsupported (Expr) , }
    };
}

Value!()