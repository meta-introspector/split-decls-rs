macro_rules! deps {
    () => {
        NopLayer!();
    };
}

macro_rules! three_layers_are_subscriber {
    () => {
        deps!();
        # [test] fn three_layers_are_subscriber () { let s = NopLayer . and_then (NopLayer) . and_then (NopLayer) . with_subscriber (NoSubscriber :: default ()) ; assert_subscriber (s) }
    };
}

three_layers_are_subscriber!()