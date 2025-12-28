macro_rules! deps {
    () => {
        StringLayer3!();
        StringLayer2!();
        StringLayer!();
    };
}

macro_rules! downcasts_to_layer {
    () => {
        deps!();
        # [test] fn downcasts_to_layer () { let s = StringLayer ("layer_1") . and_then (StringLayer2 ("layer_2")) . and_then (StringLayer3 ("layer_3")) . with_subscriber (NoSubscriber :: default ()) ; let layer = < dyn Subscriber > :: downcast_ref :: < StringLayer > (& s) . expect ("layer 1 should downcast") ; assert_eq ! (layer . 0 , "layer_1") ; let layer = < dyn Subscriber > :: downcast_ref :: < StringLayer2 > (& s) . expect ("layer 2 should downcast") ; assert_eq ! (layer . 0 , "layer_2") ; let layer = < dyn Subscriber > :: downcast_ref :: < StringLayer3 > (& s) . expect ("layer 3 should downcast") ; assert_eq ! (layer . 0 , "layer_3") ; }
    };
}

downcasts_to_layer!()