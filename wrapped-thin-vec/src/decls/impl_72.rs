macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < 'a , T > Iterator for Drain < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { self . iter . next () . map (| x | unsafe { ptr :: read (x) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_72!();