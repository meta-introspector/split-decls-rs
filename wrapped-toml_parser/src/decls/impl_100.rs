macro_rules! deps {
    () => {
        StringBuilder!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < 's > StringBuilder < 's > for () { fn clear (& mut self) { } fn push_str (& mut self , _append : & 's str) -> bool { true } fn push_char (& mut self , _append : char) -> bool { true } }
    };
}

impl_100!()