macro_rules! deps {
    () => {
        IndexIter!();
        RawIndex!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < const N : usize > Iterator for IndexIter < N > { type Item = RawIndex < N > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if self . remaining > 0 { self . remaining -= 1 ; Some (self . left_index . inc ()) } else { None } } # [inline] # [must_use] fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
    };
}

impl_140!()