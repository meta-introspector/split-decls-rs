macro_rules! deps {
    () => {
        NopLayer!();
    };
}

macro_rules! layer_is_subscriber {
    () => {
        deps!();
        # [test] fn layer_is_subscriber () { let s = NopLayer . with_subscriber (NoSubscriber :: default ()) ; assert_subscriber (s) }
    };
}

layer_is_subscriber!();