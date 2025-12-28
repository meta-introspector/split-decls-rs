macro_rules! deps {
    () => {
        UniCase!();
    };
}

macro_rules! into_impl {
    () => {
        deps!();
        macro_rules ! into_impl { ($ to : ty) => { impl <'a > Into <$ to > for UniCase <$ to > { fn into (self) -> $ to { self . into_inner () } } } ; }
    };
}

into_impl!()