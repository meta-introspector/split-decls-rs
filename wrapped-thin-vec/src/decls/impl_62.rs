macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < T > { if self . start == self . vec . len () { None } else { unsafe { let old_start = self . start ; self . start += 1 ; Some (ptr :: read (self . vec . data_raw () . add (old_start))) } } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . vec . len () - self . start ; (len , Some (len)) } }
    };
}

impl_62!();