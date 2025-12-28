macro_rules! deps {
    () => {
        Layer!();
    };
}

macro_rules! assert_layer {
    () => {
        deps!();
        fn assert_layer < S : Subscriber > (_l : & impl Layer < S >) { }
    };
}

assert_layer!();