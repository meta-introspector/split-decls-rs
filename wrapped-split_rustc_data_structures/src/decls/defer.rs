macro_rules! deps {
    () => {
        OnDrop!();
    };
}

macro_rules! defer {
    () => {
        deps!();
        # [doc = " Returns a structure that calls `f` when dropped."] pub fn defer < F : FnOnce () > (f : F) -> OnDrop < F > { OnDrop (Some (f)) }
    };
}

defer!();