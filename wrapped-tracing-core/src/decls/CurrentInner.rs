macro_rules! deps {
    () => {
        Id!();
        Metadata!();
        Current!();
    };
}

macro_rules! CurrentInner {
    () => {
        deps!();
        # [derive (Debug)] enum CurrentInner { Current { id : Id , metadata : & 'static Metadata < 'static > , } , None , Unknown , }
    };
}

CurrentInner!()