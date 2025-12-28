macro_rules! TrivialDrop {
    () => {
        pub (crate) trait TrivialDrop { }
    };
}

TrivialDrop!()