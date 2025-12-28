macro_rules! deps {
    () => {
        DontDropMe!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl Drop for DontDropMe { fn drop (& mut self) { test_println ! ("-> DontDropMe drop: dropping data {:?}" , self . 0 . id) ; self . 0 . is_dropped . store (true , Ordering :: SeqCst) } }
    };
}

impl_220!()