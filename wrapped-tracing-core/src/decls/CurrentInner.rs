macro_rules! deps {
    () => {
        Metadata!();
        Id!();
        Current!();
    };
}

macro_rules! CurrentInner {
    () => {
        deps!();
        # [derive (Debug)] enum CurrentInner { Current { id : Id , metadata : & 'static Metadata < 'static > , } , None , Unknown , }
    };
}

CurrentInner!();