macro_rules! read {
    () => {
        fn read (path : & AbsPath) -> Option < Vec < u8 > > { std :: fs :: read (path) . ok () }
    };
}

read!()