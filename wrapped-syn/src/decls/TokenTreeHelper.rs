macro_rules! TokenTreeHelper {
    () => {
        pub (crate) struct TokenTreeHelper < 'a > (pub & 'a TokenTree) ;
    };
}

TokenTreeHelper!()