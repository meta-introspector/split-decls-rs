macro_rules! TokenStreamExt {
    () => {
        pub (crate) trait TokenStreamExt { fn append (& mut self , token : TokenTree) ; }
    };
}

TokenStreamExt!();