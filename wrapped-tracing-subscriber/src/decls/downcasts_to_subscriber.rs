macro_rules! deps {
    () => {
        NopLayer!();
        StringSubscriber!();
    };
}

macro_rules! downcasts_to_subscriber {
    () => {
        deps!();
        # [test] fn downcasts_to_subscriber () { let s = NopLayer . and_then (NopLayer) . and_then (NopLayer) . with_subscriber (StringSubscriber ("subscriber")) ; let subscriber = < dyn Subscriber > :: downcast_ref :: < StringSubscriber > (& s) . expect ("subscriber should downcast") ; assert_eq ! (subscriber . 0 , "subscriber") ; }
    };
}

downcasts_to_subscriber!()