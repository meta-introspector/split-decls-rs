macro_rules! StringBuilder {
    () => {
        pub trait StringBuilder < 's > { fn clear (& mut self) ; # [must_use] fn push_str (& mut self , append : & 's str) -> bool ; # [must_use] fn push_char (& mut self , append : char) -> bool ; }
    };
}

StringBuilder!()