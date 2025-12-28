macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > Iterator for Iter < 'a , A , N > { type Item = & 'a A ; fn next (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . remaining -= 1 ; Some (unsafe { & * self . buffer . ptr (self . left_index . inc ()) }) } } # [inline] # [must_use] fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
    };
}

impl_146!()