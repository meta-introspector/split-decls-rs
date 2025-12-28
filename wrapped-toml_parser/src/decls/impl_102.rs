macro_rules! deps {
    () => {
        StringBuilder!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 's > StringBuilder < 's > for Cow < 's , str > { fn clear (& mut self) { match self { Cow :: Borrowed (s) => { s . clear () ; } Cow :: Owned (s) => s . clear () , } } fn push_str (& mut self , append : & 's str) -> bool { match self { Cow :: Borrowed (s) => { if ! s . push_str (append) { self . to_mut () . push_str (append) ; } } Cow :: Owned (s) => s . push_str (append) , } true } fn push_char (& mut self , append : char) -> bool { self . to_mut () . push (append) ; true } }
    };
}

impl_102!()