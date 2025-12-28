macro_rules! TokenStreamHelper {
    () => {
        pub (crate) struct TokenStreamHelper < 'a > (pub & 'a TokenStream) ;
    };
}

TokenStreamHelper!();