macro_rules! BoundVar {
    () => {
        pub (crate) type BoundVar = u32 ;
    };
}

BoundVar!()