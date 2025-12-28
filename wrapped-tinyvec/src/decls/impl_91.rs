macro_rules! deps {
    () => {
        SliceVecDrain!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < 'p , 's , T : Default > Iterator for SliceVecDrain < 'p , 's , T > { type Item = T ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if self . target_index != self . target_end { let out = core :: mem :: take (& mut self . parent [self . target_index]) ; self . target_index += 1 ; Some (out) } else { None } } }
    };
}

impl_91!()