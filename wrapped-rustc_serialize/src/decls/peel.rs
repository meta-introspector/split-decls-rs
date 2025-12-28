macro_rules! peel {
    () => {
        macro_rules ! peel { ($ name : ident , $ ($ other : ident ,) *) => (tuple ! { $ ($ other ,) * }) }
    };
}

peel!()