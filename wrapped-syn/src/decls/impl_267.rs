macro_rules! deps {
    () => {
        TokenStreamExt!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl TokenStreamExt for TokenStream { fn append (& mut self , token : TokenTree) { self . extend (iter :: once (token)) ; } }
    };
}

impl_267!()