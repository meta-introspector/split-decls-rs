macro_rules! KeySerializer {
    () => {
        pub (crate) struct KeySerializer < 'd > { pub (crate) dst : & 'd mut String , }
    };
}

KeySerializer!()