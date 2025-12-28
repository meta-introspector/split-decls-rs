macro_rules! deps {
    () => {
        ExpectedId!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl PartialEq for ExpectedId { fn eq (& self , other : & Self) -> bool { self . inner . load (Ordering :: Relaxed) == other . inner . load (Ordering :: Relaxed) } }
    };
}

impl_60!();