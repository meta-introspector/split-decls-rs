macro_rules! deps {
    () => {
        StringBuilder!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 's > StringBuilder < 's > for String { fn clear (& mut self) { self . clear () ; } fn push_str (& mut self , append : & 's str) -> bool { self . push_str (append) ; true } fn push_char (& mut self , append : char) -> bool { self . push (append) ; true } }
    };
}

impl_103!();