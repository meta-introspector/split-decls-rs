macro_rules! deps {
    () => {
        LinesWithTerminator!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < 'a > LinesWithTerminator < 'a > { pub fn new (data : & 'a str) -> LinesWithTerminator < 'a > { LinesWithTerminator { data } } }
    };
}

impl_342!();