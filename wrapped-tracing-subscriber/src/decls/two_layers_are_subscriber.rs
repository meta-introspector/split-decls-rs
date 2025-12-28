macro_rules! deps {
    () => {
        NopLayer!();
    };
}

macro_rules! two_layers_are_subscriber {
    () => {
        deps!();
        # [test] fn two_layers_are_subscriber () { let s = NopLayer . and_then (NopLayer) . with_subscriber (NoSubscriber :: default ()) ; assert_subscriber (s) }
    };
}

two_layers_are_subscriber!()