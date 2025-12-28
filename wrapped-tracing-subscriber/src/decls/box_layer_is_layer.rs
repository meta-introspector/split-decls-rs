macro_rules! deps {
    () => {
        NopLayer!();
        Layer!();
    };
}

macro_rules! box_layer_is_layer {
    () => {
        deps!();
        # [test] # [cfg (feature = "alloc")] fn box_layer_is_layer () { use alloc :: boxed :: Box ; let l : Box < dyn Layer < NoSubscriber > + Send + Sync > = Box :: new (NopLayer) ; assert_layer (& l) ; l . with_subscriber (NoSubscriber :: default ()) ; }
    };
}

box_layer_is_layer!()