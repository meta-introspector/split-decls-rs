macro_rules! deps {
    () => {
        AssertDropped!();
        SetDropped!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl AssertDropped { fn new (val : usize) -> (Self , SetDropped) { let dropped = std :: sync :: Arc :: new (AtomicBool :: new (false)) ; let val = SetDropped { val , dropped : dropped . clone () , } ; (Self { dropped } , val) } fn assert_dropped (& self) { assert ! (self . dropped . load (Ordering :: SeqCst) , "value should have been dropped!") ; } }
    };
}

impl_256!()