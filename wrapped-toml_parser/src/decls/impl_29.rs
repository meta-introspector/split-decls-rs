macro_rules! deps {
    () => {
        Raw!();
        Span!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Span { pub fn new_unchecked (start : usize , end : usize) -> Self { Self { start , end } } pub fn is_empty (& self) -> bool { self . end <= self . start } pub fn len (& self) -> usize { self . end - self . start } pub fn start (& self) -> usize { self . start } pub fn end (& self) -> usize { self . end } pub fn before (& self) -> Self { Self :: new_unchecked (self . start , self . start) } pub fn after (& self) -> Self { Self :: new_unchecked (self . end , self . end) } # [doc = " Extend this `Raw` to the end of `after`"] # [must_use] pub fn append (& self , after : Self) -> Self { Self :: new_unchecked (self . start , after . end) } }
    };
}

impl_29!()