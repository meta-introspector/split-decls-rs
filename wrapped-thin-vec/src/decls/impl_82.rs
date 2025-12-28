macro_rules! deps {
    () => {
        Splice!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < I : Iterator > Iterator for Splice < '_ , I > { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . drain . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . drain . size_hint () } }
    };
}

impl_82!();