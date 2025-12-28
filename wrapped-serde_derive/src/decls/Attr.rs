macro_rules! deps {
    () => {
        Symbol!();
        Ctxt!();
    };
}

macro_rules! Attr {
    () => {
        deps!();
        pub (crate) struct Attr < 'c , T > { cx : & 'c Ctxt , name : Symbol , tokens : TokenStream , value : Option < T > , }
    };
}

Attr!();