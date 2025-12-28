macro_rules! deps {
    () => {
        Registry!();
    };
}

macro_rules! THE_REGISTRY {
    () => {
        deps!();
        # [doc = " ////////////////////////////////////////////////////////////////////////"] # [doc = " Initialization"] static mut THE_REGISTRY : Option < Arc < Registry > > = None ;
    };
}

THE_REGISTRY!();