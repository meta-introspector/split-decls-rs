macro_rules! IterImpl {
    () => {
        pub struct IterImpl { stack : Vec < token_stream :: IntoIter > , peeked : Option < TokenTree > , }
    };
}

IterImpl!();