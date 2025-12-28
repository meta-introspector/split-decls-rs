macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > Iterator for IterMut < 'a , A , N > where A : 'a , { type Item = & 'a mut A ; fn next (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . remaining -= 1 ; let index = self . left_index . inc () ; Some (unsafe { & mut * self . mut_ptr (index) }) } } # [inline] # [must_use] fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
    };
}

impl_152!();