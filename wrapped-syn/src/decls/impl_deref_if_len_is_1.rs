macro_rules! impl_deref_if_len_is_1 {
    () => {
        macro_rules ! impl_deref_if_len_is_1 { ($ name : ident / 1) => { impl Deref for $ name { type Target = WithSpan ; fn deref (& self) -> & Self :: Target { unsafe { &* (self as * const Self) . cast ::< WithSpan > () } } } impl DerefMut for $ name { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { & mut * (self as * mut Self) . cast ::< WithSpan > () } } } } ; ($ name : ident /$ len : literal) => { } ; }
    };
}

impl_deref_if_len_is_1!();