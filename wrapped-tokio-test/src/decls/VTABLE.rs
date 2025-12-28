macro_rules! VTABLE {
    () => {
        static VTABLE : RawWakerVTable = RawWakerVTable :: new (clone , wake , wake_by_ref , drop_waker) ;
    };
}

VTABLE!()