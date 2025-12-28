macro_rules! deps {
    () => {
        StringBuilder!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < 's > StringBuilder < 's > for & 's str { fn clear (& mut self) { * self = & self [0 .. 0] ; } fn push_str (& mut self , append : & 's str) -> bool { if self . is_empty () { * self = append ; true } else { false } } fn push_char (& mut self , _append : char) -> bool { false } }
    };
}

impl_101!();