macro_rules! deps {
    () => {
        IterImpl!();
    };
}

macro_rules! new {
    () => {
        deps!();
        pub fn new (tokens : TokenStream) -> IterImpl { IterImpl { stack : vec ! [tokens . into_iter ()] , peeked : None , } }
    };
}

new!();