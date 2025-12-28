macro_rules! deps {
    () => {
        DontDropMe!();
        Clear!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl Clear for DontDropMe { fn clear (& mut self) { test_println ! ("-> DontDropMe clear: clearing data {:?}" , self . 0 . id) ; self . 0 . is_cleared . store (true , Ordering :: SeqCst) ; } }
    };
}

impl_221!()