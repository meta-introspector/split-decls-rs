macro_rules! RawIndex {
    () => {
        pub (crate) struct RawIndex < const N : usize > (usize) ;
    };
}

RawIndex!()