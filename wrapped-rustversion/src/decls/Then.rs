macro_rules! Then {
    () => {
        pub enum Then { Const (Span) , Attribute (TokenStream) , }
    };
}

Then!()