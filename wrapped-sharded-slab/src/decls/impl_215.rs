macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl State { fn assert_clear (& self) { assert ! (! self . is_dropped . load (Ordering :: SeqCst)) ; assert ! (self . is_cleared . load (Ordering :: SeqCst)) ; } fn assert_not_clear (& self) { assert ! (! self . is_dropped . load (Ordering :: SeqCst)) ; assert ! (! self . is_cleared . load (Ordering :: SeqCst)) ; } }
    };
}

impl_215!()