macro_rules! deps {
    () => {
        NopLayer!();
    };
}

macro_rules! three_layers_are_layer {
    () => {
        deps!();
        # [test] fn three_layers_are_layer () { let layers = NopLayer . and_then (NopLayer) . and_then (NopLayer) ; assert_layer (& layers) ; let _ = layers . with_subscriber (NoSubscriber :: default ()) ; }
    };
}

three_layers_are_layer!();