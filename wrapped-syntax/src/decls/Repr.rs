macro_rules! Repr {
    () => {
        pub (crate) enum Repr < 'a > { Borrowed (& 'a str) , Owned (GreenToken) , }
    };
}

Repr!()