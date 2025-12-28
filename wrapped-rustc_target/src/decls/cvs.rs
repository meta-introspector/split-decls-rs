macro_rules! cvs {
    () => {
        # [doc = " Cow-Vec-Str: Cow<'static, [Cow<'static, str>]>"] macro_rules ! cvs { () => { :: std :: borrow :: Cow :: Borrowed (& []) } ; ($ ($ x : expr) ,+ $ (,) ?) => { :: std :: borrow :: Cow :: Borrowed (& [$ (:: std :: borrow :: Cow :: Borrowed ($ x) ,) *]) } ; }
    };
}

cvs!()