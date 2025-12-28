macro_rules! deps {
    () => {
        Page!();
    };
}

macro_rules! DummySlot {
    () => {
        deps!();
        # [doc = " A placeholder type representing the slots of an uninitialized `Page`."] struct DummySlot ;
    };
}

DummySlot!()